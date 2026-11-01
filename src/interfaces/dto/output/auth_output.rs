use serde::{Deserialize, Serialize};

/// 登录成功响应数据
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user_id: String,
    pub username: String,
    pub real_name: Option<String>,
    pub role: String,
    pub token: String,
}
