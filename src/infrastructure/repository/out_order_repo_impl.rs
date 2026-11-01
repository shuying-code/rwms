// ============================================
// 出库单仓储 SeaORM 实现
// ============================================

use async_trait::async_trait;  // 异步 trait 宏
use sea_orm::{EntityTrait, QueryOrder, PaginatorTrait, ActiveModelTrait, DatabaseConnection, DatabaseTransaction, Set};
// EntityTrait: 提供 find / find_by_id 等查询方法
// QueryOrder: ORDER BY 排序
// PaginatorTrait: 分页查询
// ActiveModelTrait: insert / update / delete 等写操作
// DatabaseConnection: SeaORM 数据库连接
// DatabaseTransaction: SeaORM 数据库事务
// Set: ActiveModel 字段赋值包装器

use crate::domain::aggregate::out_order::OutOrder;                          // 领域实体：出库单
use crate::domain::aggregate::order_status::OrderStatus;                    // 领域枚举：订单状态
use crate::domain::aggregate::order_type::OrderType;                        // 领域枚举：订单类型
use crate::domain::aggregate::contact::Contact;                             // 领域值对象：联系人
use crate::domain::aggregate::phone::Phone;                                 // 领域值对象：电话
use crate::domain::repository::out_order_repo::OutOrderRepository;       // 领域 trait：出库单仓储接口
use crate::domain::repository::{RepositoryError, RepositoryResult, TransactionContext};      // 仓储层错误类型与 Result 别名

use super::super::model::out_order as sea_out_order;  // SeaORM 自动生成的出库单表 Model 模块

/// 基于 SeaORM 的出库单仓储实现
/// 持有数据库连接，实现 OutOrderRepository trait
pub struct SeaOrmOutOrderRepository {
    db: DatabaseConnection,  // SeaORM 数据库连接实例
}

impl SeaOrmOutOrderRepository {
    /// 构造函数，接收数据库连接创建仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// === ORM Model → 领域实体 转换 ===
impl From<sea_out_order::Model> for OutOrder {
    fn from(m: sea_out_order::Model) -> Self {
        OutOrder::reconstitute(
            m.order_id,
            m.warehouse_id,
            m.customer_id,
            m.operator_id,
            m.contact.map(|s| Contact::new(s).unwrap()),                                 // Option<String> → Option<Contact>
            m.phone.map(|s| Phone::new(s).unwrap()),                                     // Option<String> → Option<Phone>
            OrderStatus::from_str(&m.order_status).unwrap_or(OrderStatus::Pending),
            m.order_type.as_deref().and_then(OrderType::from_str).unwrap_or(OrderType::Outbound), // Option<String> → OrderType
            m.out_time,
            m.complete_time,
            m.remark,
            Vec::new(),   // items 需单独查询后通过 set_items 设置
        )
    }
}

// === 领域实体 → ORM ActiveModel 转换 ===
impl From<OutOrder> for sea_out_order::ActiveModel {
    fn from(o: OutOrder) -> Self {
        let order_status = o.order_status().to_string();       // 在部分移动前提取订单状态
        let complete_time = o.complete_time().clone();         // 在部分移动前提取完成时间
        sea_out_order::ActiveModel {
            order_id: Set(o.order_id().to_string()),           // Set 包装表示该字段已被显式赋值
            warehouse_id: Set(o.warehouse_id().to_string()),   // 仓库ID
            customer_id: Set(o.customer_id()),                 // 客户ID
            operator_id: Set(o.operator_id().to_string()),     // 操作员ID
            contact: Set(o.contact().clone().map(|c| c.as_str().to_string())),   // Option<Contact> → Option<String>
            phone: Set(o.phone().clone().map(|p| p.as_str().to_string())),         // Option<Phone> → Option<String>
            order_status: Set(order_status),                   // 订单状态转回字符串存入数据库
            order_type: Set(Some(o.order_type().as_str().to_string())),           // OrderType → Option<String>
            out_time: Set(o.out_time().clone()),               // 出库时间
            complete_time: Set(complete_time),                 // 完成时间
            remark: Set(o.remark().clone()),                   // 备注信息
        }
    }
}

#[async_trait]
impl OutOrderRepository for SeaOrmOutOrderRepository {
    /// 根据出库单ID查询单条出库单记录
    /// 返回 Option<OutOrder>，不存在则返回 None
    async fn find_by_id(&self, order_id: &str) -> RepositoryResult<Option<OutOrder>> {
        sea_out_order::Entity::find_by_id(order_id)   // SELECT * FROM out_order WHERE order_id = ?
            .one(&self.db).await                       // 取至多一条记录
            .map(|opt| opt.map(OutOrder::from))        // 若存在则转换为领域实体
            .map_err(RepositoryError::from)            // SeaORM 错误 → 仓储层错误
    }

    /// 分页查询出库单记录，按出库时间降序排列（最新记录在前）
    /// page: 页码（从0开始），page_size: 每页条数
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<OutOrder>> {
        sea_out_order::Entity::find()
            .order_by_desc(sea_out_order::Column::OutTime)   // ORDER BY out_time DESC
            .paginate(&self.db, page_size)                    // 设置每页大小
            .fetch_page(page).await                           // 获取指定页的数据
            .map(|list| list.into_iter().map(OutOrder::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 在事务中创建出库单
    /// 将领域实体转换为 ActiveModel 后在事务内插入
    /// 通常与出库明细的创建在同一个事务中执行，保证出库单与明细的原子性
    async fn create_in_txn(&self, txn: &TransactionContext, order: OutOrder) -> RepositoryResult<OutOrder> {
        let txn = txn.downcast_ref::<DatabaseTransaction>().unwrap();
        sea_out_order::ActiveModel::from(order)   // 领域实体 → ActiveModel
            .insert(txn).await                     // INSERT INTO out_order (...) 在事务内执行
            .map(OutOrder::from)                   // 插入结果 → 领域实体
            .map_err(RepositoryError::from)
    }

    /// 更新出库单记录（非事务版本）
    /// 根据主键更新，需确保 order_id 已存在
    async fn update(&self, order: OutOrder) -> RepositoryResult<OutOrder> {
        sea_out_order::ActiveModel::from(order)
            .update(&self.db).await                 // UPDATE out_order SET ... WHERE order_id = ?
            .map(OutOrder::from)
            .map_err(RepositoryError::from)
    }

    /// 在事务中更新出库单记录
    /// 与 update 逻辑相同，但使用事务连接执行
    /// 用于出库单与明细、库存等需要在同一事务中更新的场景
    async fn update_in_txn(&self, txn: &TransactionContext, order: OutOrder) -> RepositoryResult<OutOrder> {
        let txn = txn.downcast_ref::<DatabaseTransaction>().unwrap();
        sea_out_order::ActiveModel::from(order)
            .update(txn).await                      // 在事务内执行 UPDATE
            .map(OutOrder::from)
            .map_err(RepositoryError::from)
    }

    /// 根据出库单ID删除记录（非事务版本）
    /// 先查询确认存在，再执行删除操作；若不存在则静默跳过
    async fn delete_by_id(&self, order_id: &str) -> RepositoryResult<()> {
        if let Some(o) = sea_out_order::Entity::find_by_id(order_id).one(&self.db).await.map_err(RepositoryError::from)? {
            // 找到记录，转换为 ActiveModel 后删除
            let active: sea_out_order::ActiveModel = o.into();  // Model → ActiveModel
            active.delete(&self.db).await.map_err(RepositoryError::from)?;  // DELETE FROM out_order WHERE order_id = ?
        }
        // 若未找到记录则不执行删除，直接返回 Ok(())
        Ok(())
    }

    /// 在事务中根据出库单ID删除记录
    /// 与 delete_by_id 逻辑相同，但使用事务连接执行
    /// 用于出库单与明细、库存等需要在同一事务中删除的场景
    async fn delete_by_id_in_txn(&self, txn: &TransactionContext, order_id: &str) -> RepositoryResult<()> {
        let txn = txn.downcast_ref::<DatabaseTransaction>().unwrap();
        if let Some(o) = sea_out_order::Entity::find_by_id(order_id).one(txn).await.map_err(RepositoryError::from)? {
            let active: sea_out_order::ActiveModel = o.into();
            active.delete(txn).await.map_err(RepositoryError::from)?;  // 在事务内执行 DELETE
        }
        Ok(())
    }
}
