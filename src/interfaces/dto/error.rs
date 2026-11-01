// ============================================
// ServiceError HTTP 响应转换 + 重导出
// ServiceError 定义在 application 层
// 此处实现 IntoResponse（依赖 axum），供 handlers 使用
// ============================================

pub use crate::application::error::ServiceError;

use axum::response::{IntoResponse, Response};
use super::common::ApiResponse;

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = ApiResponse::<()>::error(self.error_code(), self.message().to_string());
        (status, axum::Json(body)).into_response()
    }
}
