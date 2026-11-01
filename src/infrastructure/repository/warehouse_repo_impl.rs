// ============================================
// 仓库仓储 SeaORM 实现
// ============================================

use async_trait::async_trait;  // 异步 trait 宏
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, QueryOrder, PaginatorTrait, DatabaseConnection, Set};
// EntityTrait: 提供 find / find_by_id 等查询方法
// QueryFilter: WHERE 条件过滤
// ColumnTrait: 列级操作（is_in 等）
// ActiveModelTrait: insert / update / delete 等写操作
// QueryOrder: ORDER BY 排序
// PaginatorTrait: 分页查询
// DatabaseConnection: SeaORM 数据库连接
// Set: ActiveModel 字段赋值包装器

use crate::domain::aggregate::warehouse::Warehouse;                          // 领域实体：仓库
use crate::domain::aggregate::warehouse_code::WarehouseCode;                 // 领域值对象：仓库编码
use crate::domain::repository::warehouse_repo::WarehouseRepository;       // 领域 trait：仓库仓储接口
use crate::domain::repository::{RepositoryError, RepositoryResult};       // 仓储层错误类型与 Result 别名

use super::super::model::warehouse as sea_warehouse;  // SeaORM 自动生成的仓库表 Model 模块

/// 基于 SeaORM 的仓库仓储实现
/// 持有数据库连接，实现 WarehouseRepository trait
pub struct SeaOrmWarehouseRepository {
    db: DatabaseConnection,  // SeaORM 数据库连接实例
}

impl SeaOrmWarehouseRepository {
    /// 构造函数，接收数据库连接创建仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// === ORM Model → 领域实体 转换 ===
impl From<sea_warehouse::Model> for Warehouse {
    fn from(m: sea_warehouse::Model) -> Self {
        Warehouse::reconstitute(
            m.warehouse_id,                                      // 仓库ID，主键（字符串格式）
            WarehouseCode::new(m.warehouse_code).unwrap(),       // 仓库编码（String → WarehouseCode）
            m.manager,                                           // 仓库管理员
            m.max_space,                                         // 最大容量
            m.remain_space,                                      // 剩余容量
            m.create_time,                                       // 创建时间
        )
    }
}

// === 领域实体 → ORM ActiveModel 转换 ===
impl From<Warehouse> for sea_warehouse::ActiveModel {
    fn from(w: Warehouse) -> Self {
        let remain_space = w.remain_space();             // 在部分移动前提取 remain_space
        let max_space = w.max_space();                   // 在部分移动前提取 max_space
        let create_time = w.create_time().clone();       // 在部分移动前提取 create_time
        sea_warehouse::ActiveModel {
            warehouse_id: Set(w.warehouse_id().to_string()),                   // Set 包装表示该字段已被显式赋值
            warehouse_code: Set(w.warehouse_code().as_str().to_string()),      // 仓库编码（WarehouseCode → String）
            manager: Set(w.manager().clone()),                                 // 仓库管理员
            max_space: Set(max_space),                                         // 最大容量
            remain_space: Set(remain_space),                                   // 剩余容量
            create_time: Set(create_time),                                     // 创建时间
        }
    }
}

#[async_trait]
impl WarehouseRepository for SeaOrmWarehouseRepository {
    /// 根据仓库ID查询单条仓库记录
    /// 返回 Option<Warehouse>，不存在则返回 None
    async fn find_by_id(&self, warehouse_id: &str) -> RepositoryResult<Option<Warehouse>> {
        sea_warehouse::Entity::find_by_id(warehouse_id)   // SELECT * FROM warehouse WHERE warehouse_id = ?
            .one(&self.db).await                           // 取至多一条记录
            .map(|opt| opt.map(Warehouse::from))           // 若存在则转换为领域实体
            .map_err(RepositoryError::from)                // SeaORM 错误 → 仓储层错误
    }

    /// 根据多个仓库ID批量查询
    /// 对应 SQL: SELECT * FROM warehouse WHERE warehouse_id IN (...)
    async fn find_by_ids(&self, ids: &[String]) -> RepositoryResult<Vec<Warehouse>> {
        if ids.is_empty() { return Ok(vec![]); }  // 空ID列表直接返回空结果，避免无效 SQL
        sea_warehouse::Entity::find()
            .filter(sea_warehouse::Column::WarehouseId.is_in(ids.to_vec()))  // WHERE warehouse_id IN (...)
            .all(&self.db).await
            .map(|list| list.into_iter().map(Warehouse::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 分页查询仓库记录，按仓库ID降序排列（最新记录在前）
    /// page: 页码（从0开始），page_size: 每页条数
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<Warehouse>> {
        sea_warehouse::Entity::find()
            .order_by_desc(sea_warehouse::Column::WarehouseId)   // ORDER BY warehouse_id DESC
            .paginate(&self.db, page_size)                         // 设置每页大小
            .fetch_page(page).await                                // 获取指定页的数据
            .map(|list| list.into_iter().map(Warehouse::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 统计仓库总数
    /// 对应 SQL: SELECT COUNT(*) FROM warehouse
    async fn count(&self) -> RepositoryResult<u64> {
        sea_warehouse::Entity::find()
            .count(&self.db).await
            .map_err(RepositoryError::from)
    }

    /// 创建新的仓库记录
    /// 将领域实体转换为 ActiveModel 后插入数据库，返回插入后的完整实体
    async fn create(&self, warehouse: Warehouse) -> RepositoryResult<Warehouse> {
        sea_warehouse::ActiveModel::from(warehouse)   // 领域实体 → ActiveModel
            .insert(&self.db).await                    // INSERT INTO warehouse (...)
            .map(Warehouse::from)                      // 插入结果 → 领域实体
            .map_err(RepositoryError::from)
    }

    /// 更新仓库记录
    /// 根据主键更新，需确保 warehouse_id 已存在
    async fn update(&self, warehouse: Warehouse) -> RepositoryResult<Warehouse> {
        sea_warehouse::ActiveModel::from(warehouse)
            .update(&self.db).await                    // UPDATE warehouse SET ... WHERE warehouse_id = ?
            .map(Warehouse::from)
            .map_err(RepositoryError::from)
    }

    /// 根据仓库ID删除记录
    /// 先查询确认存在，再执行删除操作；若不存在则静默跳过
    async fn delete_by_id(&self, warehouse_id: &str) -> RepositoryResult<()> {
        if let Some(w) = sea_warehouse::Entity::find_by_id(warehouse_id).one(&self.db).await.map_err(RepositoryError::from)? {
            // 找到记录，转换为 ActiveModel 后删除
            let active: sea_warehouse::ActiveModel = w.into();  // Model → ActiveModel
            active.delete(&self.db).await.map_err(RepositoryError::from)?;  // DELETE FROM warehouse WHERE warehouse_id = ?
        }
        // 若未找到记录则不执行删除，直接返回 Ok(())
        Ok(())
    }
}
