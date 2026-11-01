// ============================================
// 出库单明细仓储接口
// 支持事务内操作
// ============================================

use async_trait::async_trait;
use super::{RepositoryResult, TransactionContext};
use crate::domain::aggregate::out_order_item::OutOrderItem;

/// 出库单明细仓储 trait（Domain 层接口，Infrastructure 层实现）
#[async_trait]
#[allow(dead_code)]
pub trait OutOrderItemRepository: Send + Sync {
    async fn find_by_order_id(&self, order_id: &str) -> RepositoryResult<Vec<OutOrderItem>>;
    async fn find_by_order_id_in_txn(&self, txn: &TransactionContext, order_id: &str) -> RepositoryResult<Vec<OutOrderItem>>;
    async fn create_in_txn(&self, txn: &TransactionContext, item: OutOrderItem) -> RepositoryResult<OutOrderItem>;
    async fn delete_by_order_id(&self, order_id: &str) -> RepositoryResult<()>;
    async fn delete_by_order_id_in_txn(&self, txn: &TransactionContext, order_id: &str) -> RepositoryResult<()>;
}
