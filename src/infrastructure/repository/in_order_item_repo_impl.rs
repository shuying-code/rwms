// ============================================
// 入库单明细仓储 SeaORM 实现
// ============================================

use async_trait::async_trait;  // 异步 trait 宏
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, DatabaseConnection, DatabaseTransaction, Set};
// EntityTrait: 提供 find 等查询方法
// QueryFilter: WHERE 条件过滤
// ColumnTrait: 列级操作（eq 等）
// ActiveModelTrait: insert / delete 等写操作
// DatabaseConnection: SeaORM 数据库连接
// DatabaseTransaction: SeaORM 数据库事务，用于事务内操作
// Set: ActiveModel 字段赋值包装器

use crate::domain::aggregate::in_order_item::InOrderItem;                   // 领域实体：入库单明细
use crate::domain::repository::in_order_item_repo::InOrderItemRepository; // 领域 trait：入库单明细仓储接口
use crate::domain::repository::{RepositoryError, RepositoryResult, TransactionContext};       // 仓储层错误类型与 Result 别名

use super::super::model::in_order_item as sea_in_order_item;  // SeaORM 自动生成的入库单明细表 Model 模块

/// 基于 SeaORM 的入库单明细仓储实现
/// 持有数据库连接，实现 InOrderItemRepository trait
pub struct SeaOrmInOrderItemRepository {
    db: DatabaseConnection,  // SeaORM 数据库连接实例
}

impl SeaOrmInOrderItemRepository {
    /// 构造函数，接收数据库连接创建仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// === ORM Model → 领域实体 转换 ===
impl From<sea_in_order_item::Model> for InOrderItem {
    fn from(m: sea_in_order_item::Model) -> Self {
        InOrderItem::reconstitute(
            m.item_id,                               // 明细ID，主键
            m.order_id,                              // 所属入库单ID，外键关联入库单
            m.material_id,                           // 物资ID，外键关联物资表
            m.num,                                   // 入库数量
            m.purchase_price,                        // 采购单价
            m.sell_price,                            // 销售单价
            m.total_amount,                          // 总金额（数量 * 单价）
        )
    }
}

// === 领域实体 → ORM ActiveModel 转换 ===
impl From<InOrderItem> for sea_in_order_item::ActiveModel {
    fn from(i: InOrderItem) -> Self {
        sea_in_order_item::ActiveModel {
            item_id: Set(i.item_id()),                     // Set 包装表示该字段已被显式赋值
            order_id: Set(i.order_id().to_string()),       // 所属入库单ID
            material_id: Set(i.material_id().to_string()), // 物资ID
            num: Set(i.num()),                             // 入库数量
            purchase_price: Set(i.purchase_price()),       // 采购单价
            sell_price: Set(i.sell_price()),               // 销售单价
            total_amount: Set(i.total_amount()),           // 总金额
        }
    }
}

#[async_trait]
impl InOrderItemRepository for SeaOrmInOrderItemRepository {
    /// 根据入库单ID查询该单下所有明细
    /// 对应 SQL: SELECT * FROM in_order_item WHERE order_id = ?
    async fn find_by_order_id(&self, order_id: &str) -> RepositoryResult<Vec<InOrderItem>> {
        sea_in_order_item::Entity::find()
            .filter(sea_in_order_item::Column::OrderId.eq(order_id))  // WHERE order_id = ?
            .all(&self.db).await                                      // 获取全部匹配结果
            .map(|list| list.into_iter().map(InOrderItem::from).collect())  // 逐条转换为领域实体
            .map_err(RepositoryError::from)
    }

    /// 在事务中根据入库单ID查询该单下所有明细
    /// 与 find_by_order_id 逻辑相同，但使用事务连接执行查询
    /// 用于保证与其它操作在同一事务中的一致性
    async fn find_by_order_id_in_txn(&self, txn: &TransactionContext, order_id: &str) -> RepositoryResult<Vec<InOrderItem>> {
        let txn = txn.downcast_ref::<DatabaseTransaction>().unwrap();
        sea_in_order_item::Entity::find()
            .filter(sea_in_order_item::Column::OrderId.eq(order_id))  // WHERE order_id = ?
            .all(txn).await  // 使用事务连接而非普通数据库连接
            .map(|list| list.into_iter().map(InOrderItem::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 在事务中创建入库单明细
    /// 将领域实体转换为 ActiveModel 后在事务内插入
    /// 通常与入库单的创建在同一个事务中执行，保证原子性
    async fn create_in_txn(&self, txn: &TransactionContext, item: InOrderItem) -> RepositoryResult<InOrderItem> {
        let txn = txn.downcast_ref::<DatabaseTransaction>().unwrap();
        sea_in_order_item::ActiveModel::from(item)   // 领域实体 → ActiveModel
            .insert(txn).await                        // INSERT INTO in_order_item (...) 在事务内执行
            .map(InOrderItem::from)                   // 插入结果 → 领域实体
            .map_err(RepositoryError::from)
    }

    /// 根据入库单ID删除该单下所有明细（非事务版本）
    /// 对应 SQL: DELETE FROM in_order_item WHERE order_id = ?
    /// 使用 delete_many 批量删除，无需逐条查询
    async fn delete_by_order_id(&self, order_id: &str) -> RepositoryResult<()> {
        sea_in_order_item::Entity::delete_many()
            .filter(sea_in_order_item::Column::OrderId.eq(order_id))  // WHERE order_id = ?
            .exec(&self.db).await  // 执行批量删除
            .map(|_| ())           // 忽略删除结果，返回空元组
            .map_err(RepositoryError::from)
    }

    /// 在事务中根据入库单ID删除该单下所有明细
    /// 与 delete_by_order_id 逻辑相同，但使用事务连接执行
    /// 用于入库单更新时，先删旧明细再插新明细的场景
    async fn delete_by_order_id_in_txn(&self, txn: &TransactionContext, order_id: &str) -> RepositoryResult<()> {
        let txn = txn.downcast_ref::<DatabaseTransaction>().unwrap();
        sea_in_order_item::Entity::delete_many()
            .filter(sea_in_order_item::Column::OrderId.eq(order_id))  // WHERE order_id = ?
            .exec(txn).await  // 在事务内执行批量删除
            .map(|_| ())
            .map_err(RepositoryError::from)
    }
}
