// ============================================
// 仓库处理器模块
// 对应 Java 的 WarehouseController
// ============================================

use axum::{extract::{State, Query, Path}, Json}; // State=应用状态, Query=URL查询参数, Path=路径参数, Json=请求体/响应
use crate::{
    interfaces::dto::common::{ApiResponse, PageQuery},
    interfaces::dto::error::ServiceError,
    interfaces::dto::input::warehouse_input::{CreateWarehouseRequest, UpdateWarehouseRequest},
    interfaces::dto::output::warehouse_output::WarehouseResponse,
    state::AppState,
};

/// 获取仓库列表（分页）
pub async fn list_warehouses(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Query(query): Query<PageQuery>, // 从 URL 查询参数提取分页信息
) -> Result<Json<ApiResponse<Vec<WarehouseResponse>>>, ServiceError> {
    query.validate()?;
    let warehouses = state.warehouse_service.list_warehouses_paginated(query.page_index as u64 - 1, query.page_size as u64).await?;
    Ok(Json(ApiResponse::success(warehouses.into_iter().map(WarehouseResponse::from).collect())))
}

/// 获取仓库详情
pub async fn get_warehouse(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(warehouse_id): Path<String>, // 从 URL 路径提取仓库编号（如 /api/warehouses/WH001）
) -> Result<Json<ApiResponse<WarehouseResponse>>, ServiceError> {
    match state.warehouse_service.get_warehouse_by_id(&warehouse_id).await? {
        Some(w) => Ok(Json(ApiResponse::success(WarehouseResponse::from(w)))),
        None => Err(ServiceError::NotFound("仓库不存在".to_string())),
    }
}

/// 创建仓库
pub async fn create_warehouse(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Json(payload): Json<CreateWarehouseRequest>, // 从请求体提取创建参数（warehouse_code, manager, max_space, remain_space）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    if payload.warehouse_code.trim().is_empty() { // 校验仓库编码不能为空或纯空格
        return Err(ServiceError::Validation("仓库编码不能为空".to_string())); // 返回参数校验错误
    }
    // 仓库ID由前端生成或使用仓库编码作为ID
    let warehouse_id = payload.warehouse_code.clone(); // 使用仓库编码作为仓库 ID，确保编码和 ID 一致
    state.warehouse_service.save_warehouse(warehouse_id, payload.warehouse_code, payload.manager, payload.max_space, payload.remain_space).await?; // 调用仓库服务保存新仓库
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}

/// 更新仓库
pub async fn update_warehouse(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Json(payload): Json<UpdateWarehouseRequest>, // 从请求体提取更新参数（warehouse_id, warehouse_code, manager, max_space, remain_space）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    if payload.warehouse_code.trim().is_empty() { // 校验仓库编码不能为空或纯空格
        return Err(ServiceError::Validation("仓库编码不能为空".to_string())); // 返回参数校验错误
    }
    state.warehouse_service.update_warehouse(&payload.warehouse_id, payload.warehouse_code, payload.manager, payload.max_space).await?; // 调用仓库服务更新仓库信息
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}

/// 删除仓库
pub async fn delete_warehouse(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(warehouse_id): Path<String>, // 从 URL 路径提取仓库编号
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    state.warehouse_service.delete_warehouse(&warehouse_id).await?; // 调用仓库服务删除指定仓库
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}
