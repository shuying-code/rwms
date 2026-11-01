// ============================================
// Domain 领域层
// 核心业务规则，不依赖任何框架和基础设施
// 包含：聚合根/实体/值对象、仓储接口（trait）、领域服务、领域错误
// ============================================

pub mod aggregate;      // 聚合模块（聚合根、实体、值对象）
pub mod repository;     // 仓储接口模块（trait 定义，由 Infrastructure 层实现）
pub mod domain_service;  // 领域服务模块（跨聚合业务逻辑）

use thiserror::Error;

/// 领域错误（领域层抛出，应用层转换为 ServiceError）
#[derive(Debug, Error)]
pub enum DomainError {
    /// 无效操作（状态转换不合法等）
    #[error("{0}")]
    InvalidOperation(String),
    /// 库存不足
    #[error("{0}")]
    InsufficientStock(String),
    /// 业务规则违反
    #[error("{0}")]
    BusinessRuleViolation(String),
}

/// 领域操作结果类型
pub type DomainResult<T> = Result<T, DomainError>;
