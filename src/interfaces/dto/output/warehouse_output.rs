use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::domain::aggregate::warehouse::Warehouse;

/// 仓库响应 DTO
#[derive(Debug, Serialize)]
pub struct WarehouseResponse {
    pub warehouse_id: String,
    pub warehouse_code: String,
    pub manager: Option<String>,
    pub max_space: i32,
    pub remain_space: i32,
    pub create_time: Option<DateTime<Utc>>,
}

impl From<Warehouse> for WarehouseResponse {
    fn from(w: Warehouse) -> Self {
        let max_space = w.max_space();
        let remain_space = w.remain_space();
        Self {
            warehouse_id: w.warehouse_id().to_string(),
            warehouse_code: w.warehouse_code().to_string(),
            manager: w.manager().clone(),
            max_space,
            remain_space,
            create_time: w.create_time().clone(),
        }
    }
}
