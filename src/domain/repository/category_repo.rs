// ============================================
// 物资分类仓储接口
// ============================================

use async_trait::async_trait;
use super::RepositoryResult;
use crate::domain::aggregate::material_category::MaterialCategory;

/// 物资分类仓储 trait（Domain 层接口，Infrastructure 层实现）
#[async_trait]
#[allow(dead_code)]
pub trait CategoryRepository: Send + Sync {
    async fn find_by_id(&self, category_id: i32) -> RepositoryResult<Option<MaterialCategory>>;       // 根据分类ID查询单条
    async fn find_all(&self) -> RepositoryResult<Vec<MaterialCategory>>;                                // 查询全部分类
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<MaterialCategory>>;  // 分页查询
    async fn find_by_ids(&self, ids: &[i32]) -> RepositoryResult<Vec<MaterialCategory>>;               // 批量查询（用于名称关联）
    async fn create(&self, category: MaterialCategory) -> RepositoryResult<MaterialCategory>;          // 新建分类
    async fn update(&self, category: MaterialCategory) -> RepositoryResult<MaterialCategory>;          // 更新分类
    async fn delete_by_id(&self, category_id: i32) -> RepositoryResult<()>;                            // 根据ID删除
    async fn count(&self) -> RepositoryResult<u64>;                                                     // 统计总数
}
