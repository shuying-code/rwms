// ============================================
// 认证处理器模块
// 处理用户登录、登出等认证相关请求
// 对应 Java 的 LoginController
// ============================================

use axum::extract::State; // Axum State 提取器，从路由状态中获取 AppState
use axum::Json; // Axum JSON 提取器/响应类型，自动处理 JSON 序列化/反序列化
use crate::{
    interfaces::dto::common::ApiResponse,
    interfaces::dto::error::ServiceError,
    interfaces::dto::input::auth_input::LoginRequest,
    interfaces::dto::output::auth_output::LoginResponse,
    state::AppState,
};

/// 用户登录处理函数
pub async fn login(
    State(state): State<AppState>, // 从路由状态提取 AppState，包含 auth_service
    Json(payload): Json<LoginRequest>, // 从请求体提取登录参数（username + password）
) -> Result<Json<ApiResponse<LoginResponse>>, ServiceError> { // 返回成功响应或业务错误
    tracing::info!("User login attempt: {}", payload.username); // 记录登录尝试日志

    let service = &state.auth_service; // 获取认证服务实例的引用

    // 内置管理员快速登录
    if payload.username == "admin" && payload.password == "admin" { // 硬编码的超级管理员账号，密码为 admin
        let result = service.admin_login()?; // 调用管理员快速登录方法，生成 JWT 令牌
        let response = LoginResponse { // 将领域模型转换为 DTO 响应
            user_id: result.user_id, // 用户 ID
            username: result.username, // 用户名
            real_name: result.real_name, // 真实姓名
            role: result.role, // 角色
            token: result.token, // JWT 令牌
        };
        return Ok(Json(ApiResponse::success(response))); // 返回成功响应，包含登录信息
    }

    // 普通用户登录
    match service.login(&payload.username, &payload.password).await { // 调用认证服务的登录方法，查询数据库验证用户名密码
        Ok(result) => { // 登录成功
            let response = LoginResponse { // 将领域模型转换为 DTO 响应
                user_id: result.user_id, // 用户 ID
                username: result.username, // 用户名
                real_name: result.real_name, // 真实姓名
                role: result.role, // 角色
                token: result.token, // JWT 令牌
            };
            Ok(Json(ApiResponse::success(response))) // 返回成功响应
        }
        Err(ServiceError::Unauthorized(msg)) if msg.contains("已被禁用") => { // 用户被禁用的特殊处理
            Ok(Json(ApiResponse::error(403, msg))) // 返回 403 禁止访问，表示账号已被禁用
        }
        Err(ServiceError::Unauthorized(msg)) => { // 其他未授权错误（密码错误等）
            Ok(Json(ApiResponse::error(401, msg))) // 返回 401 未认证，表示用户名或密码错误
        }
        Err(e) => Err(e), // 其他错误（如数据库错误）直接向上传播
    }
}

/// 用户登出处理函数
pub async fn logout(
    State(state): State<AppState>, // 从路由状态提取 AppState
    axum::extract::Extension(claims): axum::extract::Extension<crate::infrastructure::config::jwt_utils::Claims>, // 从请求扩展中提取 JWT Claims（由认证中间件注入）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或业务错误
    state.auth_service.logout(&claims.user_id).await?; // 调用认证服务的登出方法，使 JWT 令牌失效（加入黑名单）
    Ok(Json(ApiResponse::success(()))) // 返回成功响应，无数据
}
