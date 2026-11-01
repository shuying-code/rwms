use serde::Deserialize;

/// 登录请求参数
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
