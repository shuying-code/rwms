// ============================================
// 事务工厂 trait（Domain 层抽象）
// Application 层依赖此 trait 开启事务
// Infrastructure 层提供 SeaORM 具体实现
// ============================================

use async_trait::async_trait;
use super::TransactionContext;

/// 事务工厂接口
/// Application 层通过此接口创建事务，无需知道底层数据库实现
#[async_trait]
pub trait TransactionFactory: Send + Sync {
    /// 开启一个新的事务
    async fn begin_transaction(&self) -> Result<TransactionContext, String>;
}
