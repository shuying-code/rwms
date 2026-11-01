// ============================================
// 供应商处理器模块
// 对应 Java 的 SupplierController
// ============================================

use axum::{extract::{State, Query, Path}, Json}; // State=应用状态, Query=URL查询参数, Path=路径参数, Json=请求体/响应
use crate::{
    interfaces::dto::common::{ApiResponse, PageQuery},
    interfaces::dto::error::ServiceError,
    interfaces::dto::input::supplier_input::{CreateSupplierRequest, UpdateSupplierRequest},
    interfaces::dto::output::supplier_output::SupplierResponse,
    state::AppState,
};

/// 获取供应商列表
pub async fn list_suppliers(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Query(query): Query<PageQuery>, // 从 URL 查询参数提取分页信息
) -> Result<Json<ApiResponse<Vec<SupplierResponse>>>, ServiceError> {
    query.validate()?;
    let suppliers = state.supplier_service.list_suppliers_paginated(query.page_index as u64 - 1, query.page_size as u64).await?;
    Ok(Json(ApiResponse::success(suppliers.into_iter().map(SupplierResponse::from).collect())))
}

/// 获取供应商详情
pub async fn get_supplier(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(id): Path<i32>, // 从 URL 路径提取供应商 ID（如 /api/suppliers/3）
) -> Result<Json<ApiResponse<SupplierResponse>>, ServiceError> {
    match state.supplier_service.get_supplier_by_id(id).await? {
        Some(s) => Ok(Json(ApiResponse::success(SupplierResponse::from(s)))),
        None => Err(ServiceError::NotFound("供应商不存在".to_string())),
    }
}

/// 创建供应商
pub async fn create_supplier(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Json(payload): Json<CreateSupplierRequest>, // 从请求体提取创建参数（supplier_name, contact, phone, remark）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    if payload.supplier_name.trim().is_empty() { // 校验供应商名称不能为空或纯空格
        return Err(ServiceError::Validation("供应商名称不能为空".to_string())); // 返回参数校验错误
    }
    state.supplier_service.save_supplier(payload.supplier_name, payload.contact, payload.phone, payload.remark).await?; // 调用供应商服务保存新供应商
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}

/// 删除供应商
pub async fn delete_supplier(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(id): Path<i32>, // 从 URL 路径提取供应商 ID
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    state.supplier_service.delete_supplier(id).await?; // 调用供应商服务删除指定供应商
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}

/// 更新供应商
pub async fn update_supplier(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Json(payload): Json<UpdateSupplierRequest>, // 从请求体提取更新参数（supplier_id, supplier_name, contact, phone, remark）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    if payload.supplier_name.trim().is_empty() { // 校验供应商名称不能为空或纯空格
        return Err(ServiceError::Validation("供应商名称不能为空".to_string())); // 返回参数校验错误
    }
    state.supplier_service.update_supplier(payload.supplier_id, payload.supplier_name, payload.contact, payload.phone, payload.remark).await?; // 调用供应商服务更新供应商信息
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}
