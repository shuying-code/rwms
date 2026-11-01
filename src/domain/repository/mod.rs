// ============================================
// 仓储接口模块
// Domain 层定义仓储 trait，Infrastructure 层提供具体实现
// 仓储返回领域聚合，不暴露 ORM 模型
// 领域层不依赖任何 ORM 框架
// ============================================

/// 事务上下文（Domain 层抽象）
/// 领域层仅声明此类型作为方法参数，不关心具体实现
/// 基础设施层负责创建和提交/回滚
pub struct TransactionContext {
    inner: Box<dyn TransactionInner + Send + Sync>,
}

/// 事务内部行为 trait（基础设施层实现）
#[async_trait::async_trait]
pub(crate) trait TransactionInner: Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
    async fn commit(self: Box<Self>) -> Result<(), String>;
}

impl TransactionContext {
    /// 创建事务上下文（由基础设施层调用）
    pub fn new(inner: impl TransactionInner + 'static) -> Self {
        Self { inner: Box::new(inner) }
    }

    /// 获取内部事务对象的引用（仅基础设施层仓储实现使用）
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.inner.as_any().downcast_ref::<T>()
    }

    /// 提交事务
    pub async fn commit(self) -> Result<(), String> {
        self.inner.commit().await
    }
}

/// 仓储层统一错误类型（Domain 层定义，不依赖具体 ORM 实现）
#[derive(Debug)]
#[allow(dead_code)]
pub enum RepositoryError {
    NotFound(String),                          // 资源不存在
    PersistenceError(String),                  // 持久化操作错误
}

/// 仓储操作结果类型
pub type RepositoryResult<T> = Result<T, RepositoryError>;

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "资源不存在: {}", msg),       // 资源不存在的错误描述
            Self::PersistenceError(msg) => write!(f, "持久化错误: {}", msg),  // 持久化错误的错误描述
        }
    }
}

pub mod transaction_factory;
pub use transaction_factory::TransactionFactory;

pub mod user_repo;              // 用户仓储接口
pub mod category_repo;          // 物资分类仓储接口
pub mod customer_repo;          // 客户仓储接口
pub mod supplier_repo;          // 供应商仓储接口
pub mod warehouse_repo;         // 仓库仓储接口
pub mod material_repo;          // 物资仓储接口
pub mod stock_repo;             // 库存仓储接口（支持事务内操作）
pub mod in_order_repo;          // 入库单仓储接口（支持事务内操作）
pub mod in_order_item_repo;     // 入库单明细仓储接口（支持事务内操作）
pub mod out_order_repo;         // 出库单仓储接口（支持事务内操作）
pub mod out_order_item_repo;    // 出库单明细仓储接口（支持事务内操作）
