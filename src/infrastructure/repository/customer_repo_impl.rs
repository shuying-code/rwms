// ============================================
// 客户仓储 SeaORM 实现
// ============================================

use async_trait::async_trait;  // 异步 trait 宏
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, QueryOrder, PaginatorTrait, DatabaseConnection, Set};
// EntityTrait: 提供 find / find_by_id 等查询方法
// QueryFilter: WHERE 条件过滤
// ColumnTrait: 列级操作（eq, is_in 等）
// ActiveModelTrait: insert / update / delete 等写操作
// QueryOrder: ORDER BY 排序
// PaginatorTrait: 分页查询
// DatabaseConnection: SeaORM 数据库连接
// Set: ActiveModel 字段赋值包装器

use crate::domain::aggregate::customer::Customer;          // 领域实体：客户
use crate::domain::aggregate::contact::Contact;            // 领域值对象：联系人
use crate::domain::aggregate::phone::Phone;                // 领域值对象：电话
use crate::domain::repository::customer_repo::CustomerRepository;  // 领域 trait：客户仓储接口
use crate::domain::repository::{RepositoryError, RepositoryResult};  // 仓储层错误类型与 Result 别名

use super::super::model::customer as sea_customer;  // SeaORM 自动生成的客户表 Model 模块

/// 基于 SeaORM 的客户仓储实现
/// 持有数据库连接，实现 CustomerRepository trait
pub struct SeaOrmCustomerRepository {
    db: DatabaseConnection,  // SeaORM 数据库连接实例
}

impl SeaOrmCustomerRepository {
    /// 构造函数，接收数据库连接创建仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// === ORM Model → 领域实体 转换 ===
impl From<sea_customer::Model> for Customer {
    fn from(m: sea_customer::Model) -> Self {
        Customer::reconstitute(
            m.customer_id,                                                          // 客户ID，主键
            m.customer_name,                                                        // 客户名称
            m.contact.map(|s| Contact::new(s).unwrap()),                            // 联系人（String → Contact）
            m.phone.map(|s| Phone::new(s).unwrap()),                                // 联系电话（String → Phone）
            m.remark,                                                               // 备注信息
        )
    }
}

// === 领域实体 → ORM ActiveModel 转换 ===
impl From<Customer> for sea_customer::ActiveModel {
    fn from(c: Customer) -> Self {
        sea_customer::ActiveModel {
            customer_id: Set(c.customer_id()),                                    // Set 包装表示该字段已被显式赋值
            customer_name: Set(c.customer_name().to_string()),                     // 客户名称
            contact: Set(c.contact().clone().map(|ct| ct.as_str().to_string())),  // Option<Contact> → Option<String>
            phone: Set(c.phone().clone().map(|p| p.as_str().to_string())),        // Option<Phone> → Option<String>
            remark: Set(c.remark().clone()),                                       // 备注信息
        }
    }
}

#[async_trait]
impl CustomerRepository for SeaOrmCustomerRepository {
    /// 根据客户ID查询单条客户记录
    /// 返回 Option<Customer>，不存在则返回 None
    async fn find_by_id(&self, customer_id: i32) -> RepositoryResult<Option<Customer>> {
        sea_customer::Entity::find_by_id(customer_id)  // SELECT * FROM customer WHERE customer_id = ?
            .one(&self.db).await                       // 取至多一条记录
            .map(|opt| opt.map(Customer::from))        // 若存在则转换为领域实体
            .map_err(RepositoryError::from)            // SeaORM 错误 → 仓储层错误
    }

    /// 根据多个客户ID批量查询
    /// 对应 SQL: SELECT * FROM customer WHERE customer_id IN (...)
    async fn find_by_ids(&self, ids: &[i32]) -> RepositoryResult<Vec<Customer>> {
        if ids.is_empty() { return Ok(vec![]); }  // 空ID列表直接返回空结果，避免无效 SQL
        sea_customer::Entity::find()
            .filter(sea_customer::Column::CustomerId.is_in(ids.to_vec()))  // WHERE customer_id IN (...)
            .all(&self.db).await
            .map(|list| list.into_iter().map(Customer::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 分页查询客户记录，按客户ID降序排列（最新记录在前）
    /// page: 页码（从0开始），page_size: 每页条数
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<Customer>> {
        sea_customer::Entity::find()
            .order_by_desc(sea_customer::Column::CustomerId)   // ORDER BY customer_id DESC
            .paginate(&self.db, page_size)                      // 设置每页大小
            .fetch_page(page).await                             // 获取指定页的数据
            .map(|list| list.into_iter().map(Customer::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 统计客户总数
    /// 对应 SQL: SELECT COUNT(*) FROM customer
    async fn count(&self) -> RepositoryResult<u64> {
        sea_customer::Entity::find()
            .count(&self.db).await
            .map_err(RepositoryError::from)
    }

    /// 创建新的客户记录
    /// 将领域实体转换为 ActiveModel 后插入数据库，返回插入后的完整实体
    async fn create(&self, customer: Customer) -> RepositoryResult<Customer> {
        sea_customer::ActiveModel::from(customer)   // 领域实体 → ActiveModel
            .insert(&self.db).await                  // INSERT INTO customer (...)
            .map(Customer::from)                     // 插入结果 → 领域实体
            .map_err(RepositoryError::from)
    }

    /// 更新客户记录
    /// 根据主键更新，需确保 customer_id 已存在
    async fn update(&self, customer: Customer) -> RepositoryResult<Customer> {
        sea_customer::ActiveModel::from(customer)
            .update(&self.db).await                  // UPDATE customer SET ... WHERE customer_id = ?
            .map(Customer::from)
            .map_err(RepositoryError::from)
    }

    /// 根据客户ID删除记录
    /// 先查询确认存在，再执行删除操作；若不存在则静默跳过
    async fn delete_by_id(&self, customer_id: i32) -> RepositoryResult<()> {
        if let Some(c) = sea_customer::Entity::find_by_id(customer_id).one(&self.db).await.map_err(RepositoryError::from)? {
            // 找到记录，转换为 ActiveModel 后删除
            let active: sea_customer::ActiveModel = c.into();  // Model → ActiveModel
            active.delete(&self.db).await.map_err(RepositoryError::from)?;  // DELETE FROM customer WHERE customer_id = ?
        }
        // 若未找到记录则不执行删除，直接返回 Ok(())
        Ok(())
    }
}
