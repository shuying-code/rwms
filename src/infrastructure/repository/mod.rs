// ============================================
// 仓储实现模块（Infrastructure 层 Repository Impl）
// SeaORM 实现 Domain 层定义的 Repository trait
// 包含持久化模型 ↔ 领域聚合的双向转换
// ============================================

pub mod user_repo_impl;       // 用户仓储实现模块
pub mod category_repo_impl;   // 物资分类仓储实现模块
pub mod customer_repo_impl;   // 客户仓储实现模块
pub mod supplier_repo_impl;   // 供应商仓储实现模块
pub mod warehouse_repo_impl;  // 仓库仓储实现模块
pub mod material_repo_impl;   // 物资仓储实现模块
pub mod stock_repo_impl;      // 库存仓储实现模块
pub mod in_order_repo_impl;   // 入库单仓储实现模块
pub mod in_order_item_repo_impl;  // 入库单明细仓储实现模块
pub mod out_order_repo_impl;      // 出库单仓储实现模块
pub mod out_order_item_repo_impl; // 出库单明细仓储实现模块

use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait, DbErr};
use crate::domain::repository::{RepositoryError, TransactionContext, TransactionInner};

impl From<DbErr> for RepositoryError {
    fn from(err: DbErr) -> Self {
        RepositoryError::PersistenceError(err.to_string())
    }
}

/// SeaORM 事务内部实现
pub struct SeaOrmTransactionInner {
    txn: DatabaseTransaction,
}

impl SeaOrmTransactionInner {
    pub fn new(txn: DatabaseTransaction) -> Self {
        Self { txn }
    }
}

#[async_trait::async_trait]
impl TransactionInner for SeaOrmTransactionInner {
    fn as_any(&self) -> &dyn std::any::Any {
        &self.txn
    }

    async fn commit(self: Box<Self>) -> Result<(), String> {
        self.txn.commit().await.map_err(|e| e.to_string())
    }
}

/// 创建事务上下文（由 TransactionFactory 内部调用）
pub(crate) fn create_transaction_context(db: &DatabaseConnection) -> impl std::future::Future<Output = Result<TransactionContext, String>> + Send {
    async move {
        let txn = db.begin().await.map_err(|e| format!("开启事务失败: {}", e))?;
        Ok(TransactionContext::new(SeaOrmTransactionInner::new(txn)))
    }
}

/// SeaORM 事务工厂（实现 Domain 层的 TransactionFactory trait）
pub struct SeaOrmTransactionFactory {
    db: DatabaseConnection,
}

impl SeaOrmTransactionFactory {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl crate::domain::repository::TransactionFactory for SeaOrmTransactionFactory {
    async fn begin_transaction(&self) -> Result<TransactionContext, String> {
        create_transaction_context(&self.db).await
    }
}
