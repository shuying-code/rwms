// ============================================
// 供应商仓储 SeaORM 实现
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

use crate::domain::aggregate::supplier::Supplier;                           // 领域实体：供应商
use crate::domain::aggregate::contact::Contact;                             // 领域值对象：联系人
use crate::domain::aggregate::phone::Phone;                                 // 领域值对象：电话
use crate::domain::repository::supplier_repo::SupplierRepository;        // 领域 trait：供应商仓储接口
use crate::domain::repository::{RepositoryError, RepositoryResult};      // 仓储层错误类型与 Result 别名

use super::super::model::supplier as sea_supplier;  // SeaORM 自动生成的供应商表 Model 模块

/// 基于 SeaORM 的供应商仓储实现
/// 持有数据库连接，实现 SupplierRepository trait
pub struct SeaOrmSupplierRepository {
    db: DatabaseConnection,  // SeaORM 数据库连接实例
}

impl SeaOrmSupplierRepository {
    /// 构造函数，接收数据库连接创建仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// === ORM Model → 领域实体 转换 ===
impl From<sea_supplier::Model> for Supplier {
    fn from(m: sea_supplier::Model) -> Self {
        Supplier::reconstitute(
            m.supplier_id,                                                          // 供应商ID，主键
            m.supplier_name,                                                        // 供应商名称
            m.contact.map(|s| Contact::new(s).unwrap()),                            // 联系人（String → Contact）
            m.phone.map(|s| Phone::new(s).unwrap()),                                // 联系电话（String → Phone）
            m.remark,                                                               // 备注信息
        )
    }
}

// === 领域实体 → ORM ActiveModel 转换 ===
impl From<Supplier> for sea_supplier::ActiveModel {
    fn from(s: Supplier) -> Self {
        sea_supplier::ActiveModel {
            supplier_id: Set(s.supplier_id()),                                    // Set 包装表示该字段已被显式赋值
            supplier_name: Set(s.supplier_name().to_string()),                     // 供应商名称
            contact: Set(s.contact().clone().map(|c| c.as_str().to_string())),    // Option<Contact> → Option<String>
            phone: Set(s.phone().clone().map(|p| p.as_str().to_string())),        // Option<Phone> → Option<String>
            remark: Set(s.remark().clone()),                                       // 备注信息
        }
    }
}

#[async_trait]
impl SupplierRepository for SeaOrmSupplierRepository {
    /// 根据供应商ID查询单条供应商记录
    /// 返回 Option<Supplier>，不存在则返回 None
    async fn find_by_id(&self, supplier_id: i32) -> RepositoryResult<Option<Supplier>> {
        sea_supplier::Entity::find_by_id(supplier_id)   // SELECT * FROM supplier WHERE supplier_id = ?
            .one(&self.db).await                         // 取至多一条记录
            .map(|opt| opt.map(Supplier::from))          // 若存在则转换为领域实体
            .map_err(RepositoryError::from)              // SeaORM 错误 → 仓储层错误
    }

    /// 根据多个供应商ID批量查询
    /// 对应 SQL: SELECT * FROM supplier WHERE supplier_id IN (...)
    async fn find_by_ids(&self, ids: &[i32]) -> RepositoryResult<Vec<Supplier>> {
        if ids.is_empty() { return Ok(vec![]); }  // 空ID列表直接返回空结果，避免无效 SQL
        sea_supplier::Entity::find()
            .filter(sea_supplier::Column::SupplierId.is_in(ids.to_vec()))  // WHERE supplier_id IN (...)
            .all(&self.db).await
            .map(|list| list.into_iter().map(Supplier::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 分页查询供应商记录，按供应商ID降序排列（最新记录在前）
    /// page: 页码（从0开始），page_size: 每页条数
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<Supplier>> {
        sea_supplier::Entity::find()
            .order_by_desc(sea_supplier::Column::SupplierId)   // ORDER BY supplier_id DESC
            .paginate(&self.db, page_size)                       // 设置每页大小
            .fetch_page(page).await                              // 获取指定页的数据
            .map(|list| list.into_iter().map(Supplier::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 统计供应商总数
    /// 对应 SQL: SELECT COUNT(*) FROM supplier
    async fn count(&self) -> RepositoryResult<u64> {
        sea_supplier::Entity::find()
            .count(&self.db).await
            .map_err(RepositoryError::from)
    }

    /// 创建新的供应商记录
    /// 将领域实体转换为 ActiveModel 后插入数据库，返回插入后的完整实体
    async fn create(&self, supplier: Supplier) -> RepositoryResult<Supplier> {
        sea_supplier::ActiveModel::from(supplier)   // 领域实体 → ActiveModel
            .insert(&self.db).await                  // INSERT INTO supplier (...)
            .map(Supplier::from)                     // 插入结果 → 领域实体
            .map_err(RepositoryError::from)
    }

    /// 更新供应商记录
    /// 根据主键更新，需确保 supplier_id 已存在
    async fn update(&self, supplier: Supplier) -> RepositoryResult<Supplier> {
        sea_supplier::ActiveModel::from(supplier)
            .update(&self.db).await                  // UPDATE supplier SET ... WHERE supplier_id = ?
            .map(Supplier::from)
            .map_err(RepositoryError::from)
    }

    /// 根据供应商ID删除记录
    /// 先查询确认存在，再执行删除操作；若不存在则静默跳过
    async fn delete_by_id(&self, supplier_id: i32) -> RepositoryResult<()> {
        if let Some(s) = sea_supplier::Entity::find_by_id(supplier_id).one(&self.db).await.map_err(RepositoryError::from)? {
            // 找到记录，转换为 ActiveModel 后删除
            let active: sea_supplier::ActiveModel = s.into();  // Model → ActiveModel
            active.delete(&self.db).await.map_err(RepositoryError::from)?;  // DELETE FROM supplier WHERE supplier_id = ?
        }
        // 若未找到记录则不执行删除，直接返回 Ok(())
        Ok(())
    }
}
