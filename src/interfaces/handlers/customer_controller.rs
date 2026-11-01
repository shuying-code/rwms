// ============================================
// 客户处理器模块
// 对应 Java 的 CustomerController
// ============================================

use axum::{extract::{State, Query, Path}, Json}; // State=应用状态, Query=URL查询参数, Path=路径参数, Json=请求体/响应
use crate::{
    interfaces::dto::common::{ApiResponse, PageQuery},
    interfaces::dto::error::ServiceError,
    interfaces::dto::input::customer_input::{CreateCustomerRequest, UpdateCustomerRequest},
    interfaces::dto::output::customer_output::CustomerResponse,
    state::AppState,
};

/// 获取客户列表
pub async fn list_customers(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Query(query): Query<PageQuery>, // 从 URL 查询参数提取分页信息（page_index, page_size）
) -> Result<Json<ApiResponse<Vec<CustomerResponse>>>, ServiceError> {
    query.validate()?;
    let customers = state.customer_service.list_customers_paginated(query.page_index as u64 - 1, query.page_size as u64).await?;
    Ok(Json(ApiResponse::success(customers.into_iter().map(CustomerResponse::from).collect())))
}

/// 获取客户详情
pub async fn get_customer(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(id): Path<i32>, // 从 URL 路径提取客户 ID（如 /api/customers/3）
) -> Result<Json<ApiResponse<CustomerResponse>>, ServiceError> {
    match state.customer_service.get_customer_by_id(id).await? {
        Some(c) => Ok(Json(ApiResponse::success(CustomerResponse::from(c)))),
        None => Err(ServiceError::NotFound("客户不存在".to_string())),
    }
}

/// 创建客户
pub async fn create_customer(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Json(payload): Json<CreateCustomerRequest>, // 从请求体提取创建参数（customer_name, contact, phone, remark）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    if payload.customer_name.trim().is_empty() { // 校验客户名称不能为空或纯空格
        return Err(ServiceError::Validation("客户名称不能为空".to_string())); // 返回参数校验错误
    }
    state.customer_service.save_customer(payload.customer_name, payload.contact, payload.phone, payload.remark).await?; // 调用客户服务保存新客户
    Ok(Json(ApiResponse::success(()))) // 返回成功响应，无数据
}

/// 删除客户
pub async fn delete_customer(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(id): Path<i32>, // 从 URL 路径提取客户 ID（如 /api/customers/3）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    state.customer_service.delete_customer(id).await?; // 调用客户服务删除指定客户
    Ok(Json(ApiResponse::success(()))) // 返回成功响应，无数据
}

/// 更新客户
pub async fn update_customer(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Json(payload): Json<UpdateCustomerRequest>, // 从请求体提取更新参数（customer_id, customer_name, contact, phone, remark）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    if payload.customer_name.trim().is_empty() { // 校验客户名称不能为空或纯空格
        return Err(ServiceError::Validation("客户名称不能为空".to_string())); // 返回参数校验错误
    }
    state.customer_service.update_customer(payload.customer_id, payload.customer_name, payload.contact, payload.phone, payload.remark).await?; // 调用客户服务更新客户信息
    Ok(Json(ApiResponse::success(()))) // 返回成功响应，无数据
}
