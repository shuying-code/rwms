// ============================================
// 出库单仓储接口
// 支持事务内操作
// ============================================

use async_trait::async_trait;
use super::{RepositoryResult, TransactionContext};
use crate::domain::aggregate::out_order::OutOrder;

/// 出库单仓储 trait（Domain 层接口，Infrastructure 层实现）
#[async_trait]
#[allow(dead_code)]
pub trait OutOrderRepository: Send + Sync {
    async fn find_by_id(&self, order_id: &str) -> RepositoryResult<Option<OutOrder>>;
    async fn list_paginated(&self, page: u64, page_size: u64) -> RepositoryResult<Vec<OutOrder>>;
    async fn create_in_txn(&self, txn: &TransactionContext, order: OutOrder) -> RepositoryResult<OutOrder>;
    async fn update(&self, order: OutOrder) -> RepositoryResult<OutOrder>;
    async fn update_in_txn(&self, txn: &TransactionContext, order: OutOrder) -> RepositoryResult<OutOrder>;
    async fn delete_by_id(&self, order_id: &str) -> RepositoryResult<()>;
    async fn delete_by_id_in_txn(&self, txn: &TransactionContext, order_id: &str) -> RepositoryResult<()>;
}
