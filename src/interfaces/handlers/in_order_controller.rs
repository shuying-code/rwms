// ============================================
// 入库单处理器模块
// 对应 Java 的 InOrderController
// ============================================

use axum::{extract::{State, Query, Extension, Path}, Json}; // State=应用状态, Query=URL查询参数, Extension=中间件注入的Claims, Path=路径参数, Json=请求体/响应
use crate::{
    infrastructure::config::jwt_utils::Claims,
    interfaces::dto::common::{ApiResponse, PageQuery},
    interfaces::dto::error::ServiceError,
    interfaces::dto::input::in_order_input::CreateInOrderRequest,
    interfaces::dto::input::order_input::AuditOrderRequest,
    interfaces::dto::output::in_order_output::{InOrderDetailItem, InOrderListItem, InOrderItemResponse},
    state::AppState,
};

/// 获取入库单列表（分页）
pub async fn list_in_orders(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Query(query): Query<PageQuery>, // 从 URL 查询参数提取分页信息
) -> Result<Json<ApiResponse<Vec<InOrderListItem>>>, ServiceError> { // 返回入库单列表（含供应商/仓库名称）或错误
    query.validate()?; // 校验分页参数
    let items = state.in_order_service.list_in_orders_with_names(query.page_index as u64 - 1, query.page_size as u64).await?; // 调用入库单服务查询列表，含关联名称
    Ok(Json(ApiResponse::success(items))) // 返回成功响应
}

/// 获取入库单详情
pub async fn get_in_order(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(order_id): Path<String>, // 从 URL 路径提取入库单编号（如 /api/in-orders/RK202401010001）
) -> Result<Json<ApiResponse<InOrderDetailItem>>, ServiceError> { // 返回入库单详情（含明细列表）或错误
    match state.in_order_service.get_in_order_detail(&order_id).await? { // 根据订单编号查询详情
        Some(detail) => Ok(Json(ApiResponse::success(detail))), // 找到入库单，返回详情
        None => Err(ServiceError::NotFound("入库单不存在".to_string())), // 未找到，返回 404
    }
}

/// 创建入库单
pub async fn create_in_order(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Extension(claims): Extension<Claims>, // 从请求扩展提取 JWT Claims，获取当前操作员 user_id
    Json(payload): Json<CreateInOrderRequest>, // 从请求体提取入库单创建参数（订单号、仓库、供应商、明细等）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    state.in_order_service.create_in_order(&claims.user_id, &payload).await?; // 调用入库单服务创建入库单，传入操作员 ID 和请求参数
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}

/// 审核入库单
pub async fn audit_in_order(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Extension(_claims): Extension<Claims>, // 从请求扩展提取 JWT Claims（当前未使用，预留权限校验）
    Json(payload): Json<AuditOrderRequest>, // 从请求体提取审核参数（order_id, order_status）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    state.in_order_service.audit_in_order(&payload).await?; // 调用入库单服务审核，审核通过时自动增加库存
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}

/// 删除入库单
pub async fn delete_in_order(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(order_id): Path<String>, // 从 URL 路径提取入库单编号
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    state.in_order_service.delete_in_order_with_items(&order_id).await?; // 调用入库单服务级联删除订单及其明细
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}

/// 获取入库单明细列表
pub async fn list_in_order_items(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(order_id): Path<String>, // 从 URL 路径提取入库单编号（如 /api/in-orders/RK202401010001/items）
) -> Result<Json<ApiResponse<Vec<InOrderItemResponse>>>, ServiceError> {
    let items = state.in_order_service.list_in_order_items(&order_id).await?;
    Ok(Json(ApiResponse::success(items.into_iter().map(InOrderItemResponse::from).collect())))
}
