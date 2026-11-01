// ============================================
// 认证中间件模块
// 对应 Java 的 TokenInterceptor / AuthFilter
// 拦截受保护路由的请求，验证 JWT Token 的有效性
// ============================================

// Axum 核心类型：Request 是 HTTP 请求对象，Next 用于调用下一个中间件/处理器
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
// State 提取器：从路由层注入的 AppState 中获取共享状态
use axum::extract::State;
// JWT 验证函数：解码并验证 Token 的签名和过期时间
use crate::infrastructure::config::jwt_utils::verify_token;
// 应用共享状态：持有 AuthService（提供 jwt_secret）
use crate::state::AppState;

/// 认证中间件 - 验证 JWT Token
///
/// 对应 Java 的 HandlerInterceptor.preHandle()
/// 执行流程：提取 Token → 验证签名和过期时间 → 注入用户信息到请求 → 放行
///
/// # 参数
/// * `State(state)` - Axum State 提取器，获取 AppState（包含 AuthService）
/// * `req` - 当前 HTTP 请求对象（可变，因为需要向其注入 Claims）
/// * `next` - 中间件链的下一个处理器，调用 next.run(req) 放行请求
///
/// # 返回
/// * `Ok(Response)` - Token 验证通过，返回后续处理器的响应
/// * `Err((StatusCode, String))` - Token 缺失或无效，返回 401 Unauthorized
pub async fn auth_middleware(
    State(state): State<AppState>,   // 从路由层注入的 AppState，包含 auth_service
    mut req: Request,                 // 可变请求对象，需要向 extensions 中注入 Claims
    next: Next,                       // 中间件链的下一个处理器，调用 run 消费请求
) -> Result<Response, (StatusCode, String)> {  // 成功返回响应，失败返回 HTTP 状态码+错误信息
    // 引入 HTTP 标准头常量（如 AUTHORIZATION = "Authorization"）
    use axum::http::header;

    // 从请求头中提取 Bearer Token
    let token = req
        .headers()                                    // 获取请求头 Map（HeaderMap 类型）
        .get(header::AUTHORIZATION)                   // 查找 Authorization 头（不区分大小写）
        .and_then(|value| value.to_str().ok())        // 将 HeaderValue 转为 &str（非 ASCII 则跳过，返回 None）
        .and_then(|value| value.strip_prefix("Bearer "))  // 去掉 "Bearer " 前缀，提取纯 Token 字符串
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing token".to_string()))?;  // 缺失则返回 401 错误

    // 验证 Token 的签名和过期时间
    // state.auth_service.jwt_secret() 获取 JWT 签名密钥
    let claims = verify_token(token, state.auth_service.jwt_secret())
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;  // 验证失败返回 401，错误信息不暴露详情

    // 将解析出的用户信息（Claims）注入到请求的 extensions 中
    // 后续 Handler 可通过 Extension<Claims> 提取器获取用户信息
    req.extensions_mut().insert(claims);   // insert 会覆盖同类型的已有 extension

    // 调用下一个中间件/处理器，并返回其响应
    Ok(next.run(req).await)               // next.run 消费 req，返回后续处理链的最终响应
}
