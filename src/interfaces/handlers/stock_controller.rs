// ============================================
// 库存处理器模块
// 对应 Java 的 StockController
// ============================================

use axum::{extract::{State, Query}, Json}; // State=应用状态, Query=URL查询参数, Json=请求体/响应
use crate::{
    interfaces::dto::common::ApiResponse,
    interfaces::dto::error::ServiceError,
    interfaces::dto::input::stock_input::StockQuery,
    interfaces::dto::output::stock_output::StockListItem,
    state::AppState,
};

/// 获取库存列表（支持多条件过滤和分页）
pub async fn list_stocks(
    State(state): State<AppState>, // 从路由状态提取 AppState
    Query(query): Query<StockQuery>, // 从 URL 查询参数提取库存查询条件（物资编号、仓库、时间范围、分页等）
) -> Result<Json<ApiResponse<Vec<StockListItem>>>, ServiceError> { // 返回库存列表（含物资/仓库名称）或错误
    let items = state.stock_service.list_stocks_with_names(
        query.material_code.as_deref(), // 物资编号模糊匹配，Option<String> 转为 Option<&str>
        query.warehouse_id.as_deref(), // 仓库编号过滤，None 表示查所有仓库
        query.start_date, // 入库时间下限
        query.end_date, // 入库时间上限
        query.page_index as u64 - 1, // 偏移量 = 页码 - 1
        query.page_size as u64, // 每页条数
    ).await?; // 调用库存服务查询，含关联名称
    Ok(Json(ApiResponse::success(items))) // 返回成功响应
}
