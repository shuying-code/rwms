// ============================================
// 库存仓储 SeaORM 实现
// 支持事务内操作
// ============================================

use async_trait::async_trait;  // 异步 trait 宏
use chrono::{DateTime, Utc};  // Chrono 时间库，用于日期时间过滤参数
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, QueryOrder, PaginatorTrait, DatabaseConnection, DatabaseTransaction, Set};
// EntityTrait: 提供 find 等查询方法
// QueryFilter: WHERE 条件过滤
// ColumnTrait: 列级操作（eq, contains, gte, lte 等）
// ActiveModelTrait: insert / update 等写操作
// QueryOrder: ORDER BY 排序
// PaginatorTrait: 分页查询
// DatabaseConnection: SeaORM 数据库连接
// DatabaseTransaction: SeaORM 数据库事务
// Set: ActiveModel 字段赋值包装器

use crate::domain::aggregate::stock::Stock;                                // 领域实体：库存
use crate::domain::repository::stock_repo::StockRepository;              // 领域 trait：库存仓储接口
use crate::domain::repository::{RepositoryError, RepositoryResult, TransactionContext};      // 仓储层错误类型与 Result 别名

use super::super::model::stock as sea_stock;  // SeaORM 自动生成的库存表 Model 模块

/// 基于 SeaORM 的库存仓储实现
/// 持有数据库连接，实现 StockRepository trait
/// 库存操作通常需要事务支持，因此提供了多个 in_txn 方法
pub struct SeaOrmStockRepository {
    db: DatabaseConnection,  // SeaORM 数据库连接实例
}

impl SeaOrmStockRepository {
    /// 构造函数，接收数据库连接创建仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// === ORM Model → 领域实体 转换 ===
impl From<sea_stock::Model> for Stock {
    fn from(m: sea_stock::Model) -> Self {
        Stock::reconstitute(
            m.stock_id,                     // 库存记录ID，主键
            m.material_id,                  // 物资ID，外键关联物资表
            m.warehouse_id,                 // 仓库ID，外键关联仓库表
            m.stock_num,                    // 当前库存数量
            m.in_time,                      // 首次入库时间
            m.update_time,                  // 最后更新时间
        )
    }
}

// === 领域实体 → ORM ActiveModel 转换 ===
impl From<Stock> for sea_stock::ActiveModel {
    fn from(s: Stock) -> Self {
        let stock_num = s.stock_num();                         // 在部分移动前提取 stock_num
        let update_time = s.update_time().clone();             // 在部分移动前提取 update_time
        let in_time = s.in_time().clone();                     // 在部分移动前提取 in_time
        sea_stock::ActiveModel {
            stock_id: Set(s.stock_id()),                     // Set 包装表示该字段已被显式赋值
            material_id: Set(s.material_id().to_string()),   // 物资ID
            warehouse_id: Set(s.warehouse_id().to_string()), // 仓库ID
            stock_num: Set(stock_num),                     // 当前库存数量
            in_time: Set(in_time),                       // 首次入库时间
            update_time: Set(update_time),                 // 最后更新时间
        }
    }
}

#[async_trait]
impl StockRepository for SeaOrmStockRepository {
    /// 根据物资ID和仓库ID联合查询库存记录
    /// 同一物资在不同仓库有独立的库存记录
    /// 对应 SQL: SELECT * FROM stock WHERE material_id = ? AND warehouse_id = ?
    async fn find_by_material_and_warehouse(&self, material_id: &str, warehouse_id: &str) -> RepositoryResult<Option<Stock>> {
        sea_stock::Entity::find()
            .filter(sea_stock::Column::MaterialId.eq(material_id))      // WHERE material_id = ?
            .filter(sea_stock::Column::WarehouseId.eq(warehouse_id))    // AND warehouse_id = ?
            .one(&self.db).await                                        // 取至多一条记录
            .map(|opt| opt.map(Stock::from))                            // 若存在则转换为领域实体
            .map_err(RepositoryError::from)
    }

    /// 在事务中根据物资ID和仓库ID联合查询库存记录
    /// 与 find_by_material_and_warehouse 逻辑相同，但使用事务连接执行
    /// 用于入库/出库操作中，在事务内查询当前库存以保证数据一致性
    async fn find_by_material_and_warehouse_in_txn(&self, txn: &TransactionContext, material_id: &str, warehouse_id: &str) -> RepositoryResult<Option<Stock>> {
        let txn = txn.downcast_ref::<DatabaseTransaction>().unwrap();
        sea_stock::Entity::find()
            .filter(sea_stock::Column::MaterialId.eq(material_id))      // WHERE material_id = ?
            .filter(sea_stock::Column::WarehouseId.eq(warehouse_id))    // AND warehouse_id = ?
            .one(txn).await  // 使用事务连接查询
            .map(|opt| opt.map(Stock::from))
            .map_err(RepositoryError::from)
    }

    /// 在事务中创建库存记录
    /// 入库时如果该物资在该仓库无库存记录，则新建一条
    /// 必须在事务中执行，与入库单操作保持原子性
    async fn create_in_txn(&self, txn: &TransactionContext, stock: Stock) -> RepositoryResult<Stock> {
        let txn = txn.downcast_ref::<DatabaseTransaction>().unwrap();
        sea_stock::ActiveModel::from(stock)   // 领域实体 → ActiveModel
            .insert(txn).await                 // INSERT INTO stock (...) 在事务内执行
            .map(Stock::from)                  // 插入结果 → 领域实体
            .map_err(RepositoryError::from)
    }

    /// 在事务中更新库存记录
    /// 入库/出库时更新已有库存记录的数量和更新时间
    /// 必须在事务中执行，与入库/出库单操作保持原子性
    async fn update_in_txn(&self, txn: &TransactionContext, stock: Stock) -> RepositoryResult<Stock> {
        let txn = txn.downcast_ref::<DatabaseTransaction>().unwrap();
        sea_stock::ActiveModel::from(stock)
            .update(txn).await                 // UPDATE stock SET stock_num=..., update_time=... WHERE stock_id = ? 在事务内执行
            .map(Stock::from)
            .map_err(RepositoryError::from)
    }

    /// 多条件过滤分页查询库存记录
    /// 支持按物资编码（模糊匹配）、仓库ID（精确匹配）、入库时间范围进行筛选
    /// 所有过滤条件均为可选（Option），未提供则不添加对应 WHERE 子句
    async fn list_filtered_paginated(
        &self,
        material_code: Option<&str>,     // 可选：物资编码过滤（模糊匹配）
        warehouse_id: Option<&str>,      // 可选：仓库ID过滤（精确匹配）
        start_date: Option<&DateTime<Utc>>,  // 可选：入库时间范围起始（>=）
        end_date: Option<&DateTime<Utc>>,    // 可选：入库时间范围结束（<=）
        page: u64,                       // 页码（从0开始）
        page_size: u64,                  // 每页条数
    ) -> RepositoryResult<Vec<Stock>> {
        let mut select = sea_stock::Entity::find();  // 构建 SELECT * FROM stock 查询
        if let Some(code) = material_code {
            // 物资编码模糊匹配：WHERE material_id LIKE '%code%'
            select = select.filter(sea_stock::Column::MaterialId.contains(code));
        }
        if let Some(wid) = warehouse_id {
            // 仓库ID精确匹配：AND warehouse_id = ?
            select = select.filter(sea_stock::Column::WarehouseId.eq(wid));
        }
        if let Some(start) = start_date {
            // 入库时间大于等于起始日期：AND in_time >= ?
            select = select.filter(sea_stock::Column::InTime.gte(*start));
        }
        if let Some(end) = end_date {
            // 入库时间小于等于结束日期：AND in_time <= ?
            select = select.filter(sea_stock::Column::InTime.lte(*end));
        }
        select
            .order_by_asc(sea_stock::Column::MaterialId)   // ORDER BY material_id ASC
            .paginate(&self.db, page_size)                   // 设置每页大小
            .fetch_page(page).await                          // 获取指定页的数据
            .map(|list| list.into_iter().map(Stock::from).collect())  // 逐条转换为领域实体
            .map_err(RepositoryError::from)
    }
}
