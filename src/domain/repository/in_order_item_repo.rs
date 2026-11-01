// ============================================
// 入库单明细仓储接口
// 支持事务内操作
// ============================================

use async_trait::async_trait;
use super::{RepositoryResult, TransactionContext};
use crate::domain::aggregate::in_order_item::InOrderItem;

/// 入库单明细仓储 trait（Domain 层接口，Infrastructure 层实现）
#[async_trait]
#[allow(dead_code)]
pub trait InOrderItemRepository: Send + Sync {
    async fn find_by_order_id(&self, order_id: &str) -> RepositoryResult<Vec<InOrderItem>>;
    async fn find_by_order_id_in_txn(&self, txn: &TransactionContext, order_id: &str) -> RepositoryResult<Vec<InOrderItem>>;
    async fn create_in_txn(&self, txn: &TransactionContext, item: InOrderItem) -> RepositoryResult<InOrderItem>;
    async fn delete_by_order_id(&self, order_id: &str) -> RepositoryResult<()>;
    async fn delete_by_order_id_in_txn(&self, txn: &TransactionContext, order_id: &str) -> RepositoryResult<()>;
}
