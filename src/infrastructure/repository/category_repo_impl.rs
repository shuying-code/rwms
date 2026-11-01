// ============================================
// 物资分类仓储 SeaORM 实现
// ============================================

use async_trait::async_trait;  // 异步 trait 宏，支持 async fn 在 trait 中使用
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, QueryOrder, PaginatorTrait, DatabaseConnection, Set};
// EntityTrait: 提供 find / find_by_id 等查询方法
// QueryFilter: WHERE 条件过滤
// ColumnTrait: 列级操作（eq, is_in 等）
// ActiveModelTrait: insert / update / delete 等写操作
// QueryOrder: ORDER BY 排序
// PaginatorTrait: 分页查询
// DatabaseConnection: SeaORM 数据库连接
// Set: 为 ActiveModel 字段赋值包装器

use crate::domain::aggregate::material_category::MaterialCategory;       // 领域实体：物资分类
use crate::domain::repository::category_repo::CategoryRepository;     // 领域 trait：分类仓储接口
use crate::domain::repository::{RepositoryError, RepositoryResult};   // 仓储层错误类型与 Result 别名

use super::super::model::material_category as sea_category;  // SeaORM 自动生成的数据库 Model 模块

/// 基于 SeaORM 的物资分类仓储实现
/// 持有数据库连接，实现 CategoryRepository trait
pub struct SeaOrmCategoryRepository {
    db: DatabaseConnection,  // SeaORM 数据库连接实例
}

impl SeaOrmCategoryRepository {
    /// 构造函数，接收数据库连接创建仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// === ORM Model → 领域实体 转换 ===
impl From<sea_category::Model> for MaterialCategory {
    fn from(m: sea_category::Model) -> Self {
        MaterialCategory::reconstitute(
            m.category_id,       // 分类ID，主键
            m.category_name,     // 分类名称
            m.remark,            // 备注信息
        )
    }
}

// === 领域实体 → ORM ActiveModel 转换 ===
impl From<MaterialCategory> for sea_category::ActiveModel {
    fn from(c: MaterialCategory) -> Self {
        sea_category::ActiveModel {
            category_id: Set(c.category_id()),       // Set 包装表示该字段已被显式赋值
            category_name: Set(c.category_name().to_string()),   // 分类名称
            remark: Set(c.remark().clone()),                 // 备注信息
        }
    }
}

#[async_trait]
impl CategoryRepository for SeaOrmCategoryRepository {
    /// 根据分类ID查询单条分类记录
    /// 返回 Option<MaterialCategory>，不存在则返回 None
    async fn find_by_id(&self, category_id: i32) -> RepositoryResult<Option<MaterialCategory>> {
        sea_category::Entity::find_by_id(category_id)  // 根据主键查询：SELECT * FROM material_category WHERE category_id = ?
            .one(&self.db).await                       // 取至多一条记录
            .map(|opt| opt.map(MaterialCategory::from))  // 若存在则转换为领域实体
            .map_err(RepositoryError::from)            // 将 SeaORM 错误转换为仓储层错误
    }

    /// 查询所有分类记录，按分类ID升序排列
    async fn find_all(&self) -> RepositoryResult<Vec<MaterialCategory>> {
        sea_category::Entity::find()                              // SELECT * FROM material_category
            .order_by_asc(sea_category::Column::CategoryId)       // ORDER BY category_id ASC
            .all(&self.db).await                                  // 获取全部结果
            .map(|list| list.into_iter().map(MaterialCategory::from).collect())  // 逐条转换为领域实体
            .map_err(RepositoryError::from)
    }

    /// 分页查询分类记录，按分类ID升序排列
    /// page: 页码（从0开始），page_size: 每页条数
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<MaterialCategory>> {
        sea_category::Entity::find()
            .order_by_asc(sea_category::Column::CategoryId)       // ORDER BY category_id ASC
            .paginate(&self.db, page_size)                        // 设置每页大小
            .fetch_page(page).await                               // 获取指定页的数据
            .map(|list| list.into_iter().map(MaterialCategory::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 根据多个分类ID批量查询
    /// 对应 SQL: SELECT * FROM material_category WHERE category_id IN (...)
    async fn find_by_ids(&self, ids: &[i32]) -> RepositoryResult<Vec<MaterialCategory>> {
        if ids.is_empty() { return Ok(vec![]); }  // 空ID列表直接返回空结果，避免生成无效 SQL
        sea_category::Entity::find()
            .filter(sea_category::Column::CategoryId.is_in(ids.to_vec()))  // WHERE category_id IN (...)
            .all(&self.db).await
            .map(|list| list.into_iter().map(MaterialCategory::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 创建新的分类记录
    /// 将领域实体转换为 ActiveModel 后插入数据库，返回插入后的完整实体
    async fn create(&self, category: MaterialCategory) -> RepositoryResult<MaterialCategory> {
        sea_category::ActiveModel::from(category)   // 领域实体 → ActiveModel
            .insert(&self.db).await                  // INSERT INTO material_category (...)
            .map(MaterialCategory::from)             // 插入结果（含自增ID等）→ 领域实体
            .map_err(RepositoryError::from)
    }

    /// 更新分类记录
    /// 根据主键更新，需确保 category_id 已存在
    async fn update(&self, category: MaterialCategory) -> RepositoryResult<MaterialCategory> {
        sea_category::ActiveModel::from(category)
            .update(&self.db).await                  // UPDATE material_category SET ... WHERE category_id = ?
            .map(MaterialCategory::from)
            .map_err(RepositoryError::from)
    }

    /// 根据分类ID删除记录
    /// 先查询确认存在，再执行删除操作；若不存在则静默跳过
    async fn delete_by_id(&self, category_id: i32) -> RepositoryResult<()> {
        if let Some(c) = sea_category::Entity::find_by_id(category_id).one(&self.db).await.map_err(RepositoryError::from)? {
            // 找到记录，转换为 ActiveModel 后删除
            let active: sea_category::ActiveModel = c.into();  // Model → ActiveModel
            active.delete(&self.db).await.map_err(RepositoryError::from)?;  // DELETE FROM material_category WHERE category_id = ?
        }
        // 若未找到记录则不执行删除，直接返回 Ok(())
        Ok(())
    }

    /// 统计分类总数
    /// 对应 SQL: SELECT COUNT(*) FROM material_category
    async fn count(&self) -> RepositoryResult<u64> {
        sea_category::Entity::find()
            .count(&self.db).await
            .map_err(RepositoryError::from)
    }
}
