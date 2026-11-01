// ============================================
// 物资仓储接口
// ============================================

use async_trait::async_trait;
use super::RepositoryResult;
use crate::domain::aggregate::material::Material;

/// 物资仓储 trait（Domain 层接口，Infrastructure 层实现）
#[async_trait]
pub trait MaterialRepository: Send + Sync {
    async fn find_by_id(&self, material_id: &str) -> RepositoryResult<Option<Material>>;              // 根据物资ID查询单条
    async fn find_by_ids(&self, ids: &[String]) -> RepositoryResult<Vec<Material>>;                   // 批量查询（用于名称关联）
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<Material>>;     // 分页查询
    async fn create(&self, material: Material) -> RepositoryResult<Material>;                         // 新建物资
    async fn update(&self, material: Material) -> RepositoryResult<Material>;                         // 更新物资
    async fn delete_by_id(&self, material_id: &str) -> RepositoryResult<()>;                          // 根据ID删除
}
