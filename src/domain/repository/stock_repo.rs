// ============================================
// 库存仓储接口
// 支持事务内操作和条件过滤分页
// ============================================

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use super::{RepositoryResult, TransactionContext};
use crate::domain::aggregate::stock::Stock;

/// 库存仓储 trait（Domain 层接口，Infrastructure 层实现）
#[async_trait]
pub trait StockRepository: Send + Sync {
    async fn find_by_material_and_warehouse(&self, material_id: &str, warehouse_id: &str) -> RepositoryResult<Option<Stock>>;
    async fn find_by_material_and_warehouse_in_txn(&self, txn: &TransactionContext, material_id: &str, warehouse_id: &str) -> RepositoryResult<Option<Stock>>;
    async fn create_in_txn(&self, txn: &TransactionContext, stock: Stock) -> RepositoryResult<Stock>;
    async fn update_in_txn(&self, txn: &TransactionContext, stock: Stock) -> RepositoryResult<Stock>;
    async fn list_filtered_paginated(
        &self,
        material_code: Option<&str>,
        warehouse_id: Option<&str>,
        start_date: Option<&DateTime<Utc>>,
        end_date: Option<&DateTime<Utc>>,
        page: u64,
        page_size: u64,
    ) -> RepositoryResult<Vec<Stock>>;
}
