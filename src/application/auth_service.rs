// ============================================
// 认证应用服务
// 处理用户登录、管理员登录、用户登出
// JWT token 生成逻辑内联，不依赖 infrastructure 层
// ============================================

use std::sync::Arc;
use serde::Serialize;
use crate::domain::repository::user_repo::UserRepository;
use crate::application::error::ServiceError;

/// JWT Token 载荷（仅在 auth_service 内部使用）
#[derive(Serialize)]
struct TokenClaims {
    user_id: String,
    username: String,
    role: String,
    exp: u64,
}

/// 生成 JWT Token
fn generate_token(user_id: &str, username: &str, role: &str, secret: &str, expires_in: u64) -> Result<String, String> {
    let exp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() + expires_in;
    let claims = TokenClaims {
        user_id: user_id.to_string(),
        username: username.to_string(),
        role: role.to_string(),
        exp,
    };
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("Token生成失败: {}", e))
}

/// 认证应用服务
#[derive(Clone)]
pub struct AuthApplicationService {
    user_repo: Arc<dyn UserRepository>,        // 用户仓储（trait object）
    jwt_secret: String,                        // JWT 签名密钥
}

/// 登录结果
pub struct LoginResult {
    pub user_id: String,
    pub username: String,
    pub real_name: Option<String>,
    pub role: String,
    pub token: String,
}

impl AuthApplicationService {
    /// 构造函数：接受仓储 trait object 和 JWT 密钥（由组合根注入）
    pub fn new(user_repo: Arc<dyn UserRepository>, jwt_secret: String) -> Self {
        Self { user_repo, jwt_secret }
    }

    /// 用户登录
    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResult, ServiceError> {
        let user = self.user_repo.find_by_username(username).await.map_err(ServiceError::from)?;

        let user = match user {
            Some(u) => u,
            None => return Err(ServiceError::Unauthorized("用户名或密码错误".to_string())),
        };

        if !user.is_active() {
            return Err(ServiceError::Unauthorized("该账号已被禁用".to_string()));
        }

        if !user.verify_password(password) {
            return Err(ServiceError::Unauthorized("用户名或密码错误".to_string()));
        }

        let token = generate_token(user.user_id(), user.username(), user.role().as_str(), &self.jwt_secret, 86400)
            .map_err(|e| ServiceError::Database(e))?;

        let user_id = user.user_id().to_string();
        let real_name = user.real_name().clone();
        let role = user.role().to_string();
        let username_val = user.username().to_string();

        self.user_repo.update_token(&user_id, &token).await.map_err(ServiceError::from)?;

        Ok(LoginResult {
            user_id,
            username: username_val,
            real_name,
            role,
            token,
        })
    }

    /// 获取 JWT 密钥
    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }

    /// 内置管理员登录（硬编码管理员账号，绕过数据库验证）
    pub fn admin_login(&self) -> Result<LoginResult, ServiceError> {
        let token = generate_token("admin001", "admin", "超级管理员", &self.jwt_secret, 86400)
            .map_err(|e| ServiceError::Database(e))?;

        Ok(LoginResult {
            user_id: "admin001".to_string(),
            username: "admin".to_string(),
            real_name: Some("系统管理员".to_string()),
            role: "超级管理员".to_string(),
            token,
        })
    }

    /// 用户登出
    pub async fn logout(&self, user_id: &str) -> Result<(), ServiceError> {
        if user_id == "admin001" {
            return Ok(());
        }
        self.user_repo.update_token(user_id, "999").await.map_err(ServiceError::from)?;
        Ok(())
    }
}
