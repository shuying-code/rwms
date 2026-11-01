// ============================================
// 物资处理器模块
// 对应 Java 的 MaterialController
// ============================================

use axum::{extract::{State, Query, Extension, Path}, Json}; // State=应用状态, Query=URL查询参数, Extension=中间件注入的Claims, Path=路径参数, Json=请求体/响应
use crate::{
    infrastructure::config::jwt_utils::Claims,
    interfaces::dto::common::{ApiResponse, PageQuery},
    interfaces::dto::error::ServiceError,
    interfaces::dto::input::material_input::{CreateMaterialRequest, UpdateMaterialRequest},
    interfaces::dto::output::material_output::MaterialListItem,
    state::AppState,
};

/// 获取物资列表（分页）
pub async fn list_materials(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Query(query): Query<PageQuery>, // 从 URL 查询参数提取分页信息
) -> Result<Json<ApiResponse<Vec<MaterialListItem>>>, ServiceError> { // 返回物资列表（含分类名称）或错误
    query.validate()?; // 校验分页参数
    let items = state.material_service.list_materials_with_category(query.page_index as u64 - 1, query.page_size as u64).await?; // 调用物资服务查询列表，含关联分类名称
    Ok(Json(ApiResponse::success(items))) // 返回成功响应
}

/// 获取物资详情
pub async fn get_material(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(material_id): Path<String>, // 从 URL 路径提取物资编号（如 /api/materials/WZ000052）
) -> Result<Json<ApiResponse<MaterialListItem>>, ServiceError> { // 返回物资详情（含分类名称）或错误
    match state.material_service.get_material_with_category(&material_id).await? { // 根据物资编号查询详情
        Some(item) => Ok(Json(ApiResponse::success(item))), // 找到物资，返回详情
        None => Err(ServiceError::NotFound("物资不存在".to_string())), // 未找到，返回 404
    }
}

/// 创建物资
pub async fn create_material(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Extension(_claims): Extension<Claims>, // 从请求扩展提取 JWT Claims（当前未使用，预留操作日志记录）
    Json(payload): Json<CreateMaterialRequest>, // 从请求体提取物资创建参数（编号、名称、分类、单位、价格）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    state.material_service.save_material(&payload).await?; // 调用物资服务保存新物资
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}

/// 更新物资
pub async fn update_material(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Extension(_claims): Extension<Claims>, // 从请求扩展提取 JWT Claims（当前未使用，预留操作日志记录）
    Json(payload): Json<UpdateMaterialRequest>, // 从请求体提取物资更新参数（编号、名称、分类、单位、价格）
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    state.material_service.update_material(&payload).await?; // 调用物资服务更新物资信息
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}

/// 删除物资
pub async fn delete_material(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Path(material_id): Path<String>, // 从 URL 路径提取物资编号
) -> Result<Json<ApiResponse<()>>, ServiceError> { // 返回空成功响应或错误
    state.material_service.delete_material(&material_id).await?; // 调用物资服务删除指定物资
    Ok(Json(ApiResponse::success(()))) // 返回成功响应
}
