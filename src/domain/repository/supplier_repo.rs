// ============================================
// 供应商仓储接口
// ============================================

use async_trait::async_trait;
use super::RepositoryResult;
use crate::domain::aggregate::supplier::Supplier;

/// 供应商仓储 trait（Domain 层接口，Infrastructure 层实现）
#[async_trait]
pub trait SupplierRepository: Send + Sync {
    async fn find_by_id(&self, supplier_id: i32) -> RepositoryResult<Option<Supplier>>;               // 根据供应商ID查询单条
    async fn find_by_ids(&self, ids: &[i32]) -> RepositoryResult<Vec<Supplier>>;                      // 批量查询（用于名称关联）
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<Supplier>>;     // 分页查询
    async fn count(&self) -> RepositoryResult<u64>;                                                   // 统计总数
    async fn create(&self, supplier: Supplier) -> RepositoryResult<Supplier>;                         // 新建供应商
    async fn update(&self, supplier: Supplier) -> RepositoryResult<Supplier>;                         // 更新供应商
    async fn delete_by_id(&self, supplier_id: i32) -> RepositoryResult<()>;                           // 根据ID删除
}
