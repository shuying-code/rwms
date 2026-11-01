// ============================================
// 客户仓储接口
// ============================================

use async_trait::async_trait;
use super::RepositoryResult;
use crate::domain::aggregate::customer::Customer;

/// 客户仓储 trait（Domain 层接口，Infrastructure 层实现）
#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn find_by_id(&self, customer_id: i32) -> RepositoryResult<Option<Customer>>;               // 根据客户ID查询单条
    async fn find_by_ids(&self, ids: &[i32]) -> RepositoryResult<Vec<Customer>>;                      // 批量查询（用于名称关联）
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<Customer>>;     // 分页查询
    async fn count(&self) -> RepositoryResult<u64>;                                                   // 统计总数
    async fn create(&self, customer: Customer) -> RepositoryResult<Customer>;                         // 新建客户
    async fn update(&self, customer: Customer) -> RepositoryResult<Customer>;                         // 更新客户
    async fn delete_by_id(&self, customer_id: i32) -> RepositoryResult<()>;                           // 根据ID删除
}
