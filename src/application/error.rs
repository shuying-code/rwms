// ============================================
// 统一业务错误类型（Application 层）
// 所有 Application 服务返回 Result<T, ServiceError>
// 领域层错误和仓储层错误通过 From trait 自动转换
// ============================================

use axum::http::StatusCode;
use sea_orm::DbErr;
use crate::domain::{DomainError};
use crate::domain::repository::RepositoryError;

/// 业务错误枚举
#[derive(Debug)]
pub enum ServiceError {
    NotFound(String),
    AlreadyExists(String),
    Validation(String),
    InsufficientStock(String),
    Database(String),
    Unauthorized(String),
}

impl ServiceError {
    pub fn message(&self) -> &str {
        match self {
            Self::NotFound(msg) => msg,
            Self::AlreadyExists(msg) => msg,
            Self::Validation(msg) => msg,
            Self::InsufficientStock(msg) => msg,
            Self::Database(msg) => msg,
            Self::Unauthorized(msg) => msg,
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::AlreadyExists(_) => StatusCode::CONFLICT,
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::InsufficientStock(_) => StatusCode::BAD_REQUEST,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }

    pub fn error_code(&self) -> i32 {
        match self {
            Self::NotFound(_) => 404,
            Self::AlreadyExists(_) => 409,
            Self::Validation(_) => 400,
            Self::InsufficientStock(_) => 400,
            Self::Database(_) => 500,
            Self::Unauthorized(_) => 401,
        }
    }
}

impl From<DomainError> for ServiceError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::InvalidOperation(msg) => ServiceError::Validation(msg),
            DomainError::InsufficientStock(msg) => ServiceError::InsufficientStock(msg),
            DomainError::BusinessRuleViolation(msg) => ServiceError::Validation(msg),
        }
    }
}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound(msg) => ServiceError::NotFound(msg),
            RepositoryError::PersistenceError(msg) => ServiceError::Database(msg),
        }
    }
}

impl From<DbErr> for ServiceError {
    fn from(err: DbErr) -> Self {
        let msg = err.to_string();
        if msg.contains("UNIQUE constraint failed") {
            ServiceError::AlreadyExists("数据已存在，请检查名称是否重复".to_string())
        } else {
            ServiceError::Database(format!("数据库操作失败: {}", msg))
        }
    }
}
