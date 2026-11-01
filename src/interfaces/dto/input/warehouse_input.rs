use serde::Deserialize;

/// 创建仓库请求参数
#[derive(Debug, Deserialize)]
pub struct CreateWarehouseRequest {
    pub warehouse_code: String,
    pub manager: Option<String>,
    pub max_space: i32,
    pub remain_space: i32,
}

/// 更新仓库请求参数
#[derive(Debug, Deserialize)]
pub struct UpdateWarehouseRequest {
    pub warehouse_id: String,
    pub warehouse_code: String,
    pub manager: Option<String>,
    pub max_space: i32,
}
