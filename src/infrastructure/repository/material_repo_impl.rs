// ============================================
// 物资仓储 SeaORM 实现
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

use crate::domain::aggregate::material::Material;                          // 领域实体：物资
use crate::domain::aggregate::unit::Unit;                                  // 领域值对象：计量单位
use crate::domain::repository::material_repo::MaterialRepository;       // 领域 trait：物资仓储接口
use crate::domain::repository::{RepositoryError, RepositoryResult};     // 仓储层错误类型与 Result 别名

use super::super::model::material as sea_material;  // SeaORM 自动生成的物资表 Model 模块

/// 基于 SeaORM 的物资仓储实现
/// 持有数据库连接，实现 MaterialRepository trait
pub struct SeaOrmMaterialRepository {
    db: DatabaseConnection,  // SeaORM 数据库连接实例
}

impl SeaOrmMaterialRepository {
    /// 构造函数，接收数据库连接创建仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// === ORM Model → 领域实体 转换 ===
impl From<sea_material::Model> for Material {
    fn from(m: sea_material::Model) -> Self {
        Material::reconstitute(
            m.material_id,               // 物资ID，主键（字符串格式）
            m.material_name,             // 物资名称
            m.category_id,               // 分类ID，外键关联物资分类表
            Unit::new(m.unit).unwrap(),  // 计量单位（String → Unit 值对象）
            m.purchase_price,            // 采购单价
            m.sell_price,                // 销售单价
            m.total_stock,               // 总库存数量
            m.create_time,               // 创建时间
            m.update_time,               // 最后更新时间
        )
    }
}

// === 领域实体 → ORM ActiveModel 转换 ===
impl From<Material> for sea_material::ActiveModel {
    fn from(m: Material) -> Self {
        sea_material::ActiveModel {
            material_id: Set(m.material_id().to_string()),               // Set 包装表示该字段已被显式赋值
            material_name: Set(m.material_name().to_string()),           // 物资名称
            category_id: Set(m.category_id()),                           // 分类ID
            unit: Set(m.unit().as_str().to_string()),                    // 计量单位（Unit → String）
            purchase_price: Set(m.purchase_price()),                     // 采购单价
            sell_price: Set(m.sell_price()),                             // 销售单价
            total_stock: Set(m.total_stock()),                           // 总库存数量
            create_time: Set(m.create_time().clone()),                   // 创建时间
            update_time: Set(m.update_time().clone()),                   // 最后更新时间
        }
    }
}

#[async_trait]
impl MaterialRepository for SeaOrmMaterialRepository {
    /// 根据物资ID查询单条物资记录
    /// 返回 Option<Material>，不存在则返回 None
    async fn find_by_id(&self, material_id: &str) -> RepositoryResult<Option<Material>> {
        sea_material::Entity::find_by_id(material_id)   // SELECT * FROM material WHERE material_id = ?
            .one(&self.db).await                         // 取至多一条记录
            .map(|opt| opt.map(Material::from))          // 若存在则转换为领域实体
            .map_err(RepositoryError::from)              // SeaORM 错误 → 仓储层错误
    }

    /// 根据多个物资ID批量查询
    /// 对应 SQL: SELECT * FROM material WHERE material_id IN (...)
    async fn find_by_ids(&self, ids: &[String]) -> RepositoryResult<Vec<Material>> {
        if ids.is_empty() { return Ok(vec![]); }  // 空ID列表直接返回空结果，避免无效 SQL
        sea_material::Entity::find()
            .filter(sea_material::Column::MaterialId.is_in(ids.to_vec()))  // WHERE material_id IN (...)
            .all(&self.db).await
            .map(|list| list.into_iter().map(Material::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 分页查询物资记录，按创建时间降序排列（最新记录在前）
    /// page: 页码（从0开始），page_size: 每页条数
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<Material>> {
        sea_material::Entity::find()
            .order_by_desc(sea_material::Column::CreateTime)   // ORDER BY create_time DESC
            .paginate(&self.db, page_size)                      // 设置每页大小
            .fetch_page(page).await                             // 获取指定页的数据
            .map(|list| list.into_iter().map(Material::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 创建新的物资记录
    /// 将领域实体转换为 ActiveModel 后插入数据库，返回插入后的完整实体
    async fn create(&self, material: Material) -> RepositoryResult<Material> {
        sea_material::ActiveModel::from(material)   // 领域实体 → ActiveModel
            .insert(&self.db).await                  // INSERT INTO material (...)
            .map(Material::from)                     // 插入结果 → 领域实体
            .map_err(RepositoryError::from)
    }

    /// 更新物资记录
    /// 根据主键更新，需确保 material_id 已存在
    async fn update(&self, material: Material) -> RepositoryResult<Material> {
        sea_material::ActiveModel::from(material)
            .update(&self.db).await                  // UPDATE material SET ... WHERE material_id = ?
            .map(Material::from)
            .map_err(RepositoryError::from)
    }

    /// 根据物资ID删除记录
    /// 先查询确认存在，再执行删除操作；若不存在则静默跳过
    async fn delete_by_id(&self, material_id: &str) -> RepositoryResult<()> {
        if let Some(m) = sea_material::Entity::find_by_id(material_id).one(&self.db).await.map_err(RepositoryError::from)? {
            // 找到记录，转换为 ActiveModel 后删除
            let active: sea_material::ActiveModel = m.into();  // Model → ActiveModel
            active.delete(&self.db).await.map_err(RepositoryError::from)?;  // DELETE FROM material WHERE material_id = ?
        }
        // 若未找到记录则不执行删除，直接返回 Ok(())
        Ok(())
    }
}
