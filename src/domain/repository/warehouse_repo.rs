// ============================================
// 仓库仓储接口
// ============================================

use async_trait::async_trait;
use super::RepositoryResult;
use crate::domain::aggregate::warehouse::Warehouse;

/// 仓库仓储 trait（Domain 层接口，Infrastructure 层实现）
#[async_trait]
pub trait WarehouseRepository: Send + Sync {
    async fn find_by_id(&self, warehouse_id: &str) -> RepositoryResult<Option<Warehouse>>;           // 根据仓库ID查询单条
    async fn find_by_ids(&self, ids: &[String]) -> RepositoryResult<Vec<Warehouse>>;                 // 批量查询（用于名称关联）
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<Warehouse>>;   // 分页查询
    async fn count(&self) -> RepositoryResult<u64>;                                                  // 统计总数
    async fn create(&self, warehouse: Warehouse) -> RepositoryResult<Warehouse>;                     // 新建仓库
    async fn update(&self, warehouse: Warehouse) -> RepositoryResult<Warehouse>;                     // 更新仓库
    async fn delete_by_id(&self, warehouse_id: &str) -> RepositoryResult<()>;                        // 根据ID删除
}
