// ============================================
// JWT 工具模块
// 对应 Java 的 JwtUtil / TokenProvider
// 提供 JWT Token 的生成和验证功能
// ============================================

// jsonwebtoken 库核心类型：encode/decode 用于编解码，EncodingKey/DecodingKey 用于签名/验签
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
/// JWT Claims（令牌载荷）
/// 包含用户身份信息和过期时间
/// 由 auth_service 生成 token，由 middleware 验证 token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub username: String,
    pub role: String,
    pub exp: u64,
}

/// 验证 JWT Token
///
/// 对应 Java 的 JwtUtil.verifyToken()
/// 解码并验证 Token 的签名和过期时间
///
/// # 参数
/// * `token` - 待验证的 JWT Token 字符串
/// * `secret` - 签名密钥（必须与生成时使用相同的密钥）
///
/// # 返回
/// * `Ok(Claims)` - 验证成功，返回 Token 中的用户信息
/// * `Err(Error)` - 验证失败（Token 过期、签名不匹配、格式错误等）
pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // 解码 Token，同时验证签名和过期时间
    let token_data = decode::<Claims>(
        token,                                             // 待解码的 Token 字符串（header.payload.signature）
        &DecodingKey::from_secret(secret.as_bytes()),      // 验签密钥：与生成时相同的密钥
        &Validation::default(),                            // 验证规则：默认检查 exp 过期时间、算法 HS256
    )?;  // ? 操作符：解码失败则提前返回 Err，成功返回 TokenData<Claims>
    // 提取并返回 Claims（Token 载荷中的用户信息）
    Ok(token_data.claims)   // TokenData 包含 header 和 claims，我们只需要 claims
}
