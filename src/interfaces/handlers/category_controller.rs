// ============================================
// 物资分类处理器模块
// 对应 Java 的 MaterialCategoryController
// ============================================

use axum::{extract::{State, Query, Path}, Json}; // State=应用状态, Query=URL查询参数, Path=路径参数, Json=请求体/响应
use crate::{
    interfaces::dto::common::{ApiResponse, PageQuery},
    interfaces::dto::error::ServiceError,
    interfaces::dto::input::category_input::{CreateCategoryRequest, UpdateCategoryRequest},
    interfaces::dto::output::category_output::CategoryResponse,
    state::AppState,
};

/// 获取分类列表（分页）
pub async fn list_categories(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Query(query): Query<PageQuery>, // 从 URL 查询参数提取分页信息（page_index, page_size）
) -> Result<Json<ApiResponse<Vec<CategoryResponse>>>, ServiceError> {
    query.validate()?;
    let categories = state.category_service.list_categories_paginated(query.page_index as u64 - 1, query.page_size as u64).await?;
    Ok(Json(ApiResponse::success(categories.into_iter().map(CategoryResponse::from).collect())))
}

/// 获取分类详情
pub async fn get_category(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(category_id): Path<i32>, // 从 URL 路径提取分类 ID（如 /api/categories/5）
) -> Result<Json<ApiResponse<CategoryResponse>>, ServiceError> {
    match state.category_service.get_category_by_id(category_id).await? {
        Some(c) => Ok(Json(ApiResponse::success(CategoryResponse::from(c)))),
        None => Err(ServiceError::NotFound("分类不存在".to_string())),
    }
}

/// 创建分类
pub async fn create_category(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Json(payload): Json<CreateCategoryRequest>, // 从请求体提取创建参数（category_name, remark）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    if payload.category_name.trim().is_empty() { // 校验分类名称不能为空或纯空格
        return Err(ServiceError::Validation("分类名称不能为空".to_string())); // 返回参数校验错误
    }
    state.category_service.save_category(payload.category_name, payload.remark).await?; // 调用分类服务保存新分类
    Ok(Json(ApiResponse::success(()))) // 返回成功响应，无数据
}

/// 更新分类
pub async fn update_category(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Json(payload): Json<UpdateCategoryRequest>, // 从请求体提取更新参数（category_id, category_name, remark）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    if payload.category_name.trim().is_empty() { // 校验分类名称不能为空或纯空格
        return Err(ServiceError::Validation("分类名称不能为空".to_string())); // 返回参数校验错误
    }
    state.category_service.update_category(payload.category_id, payload.category_name, payload.remark).await?; // 调用分类服务更新分类信息
    Ok(Json(ApiResponse::success(()))) // 返回成功响应，无数据
}

/// 删除分类
pub async fn delete_category(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(category_id): Path<i32>, // 从 URL 路径提取分类 ID（如 /api/categories/5）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    state.category_service.delete_category(category_id).await?; // 调用分类服务删除指定分类
    Ok(Json(ApiResponse::success(()))) // 返回成功响应，无数据
}
