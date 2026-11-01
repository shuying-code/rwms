// ============================================
// DTO 数据传输对象模块
// API 请求参数和响应数据的结构体
// ============================================

pub mod common; // 通用 DTO：ApiResponse 响应包装器、PageQuery 分页参数
pub mod error; // 统一错误类型：ServiceError 枚举，实现 IntoResponse
pub mod input; // 入参模块：所有 API 请求参数（Request / Query）
pub mod output; // 出参模块：所有 API 响应数据（Response / ListItem）
