use serde::Deserialize;
use chrono::{DateTime, Utc};

use crate::interfaces::dto::common::{default_page_index, default_page_size};

/// 库存查询参数
#[derive(Debug, Deserialize)]
pub struct StockQuery {
    pub material_code: Option<String>,
    #[allow(dead_code)]
    pub material_name: Option<String>,
    #[allow(dead_code)]
    pub category_id: Option<i32>,
    pub warehouse_id: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    #[serde(default = "default_page_index")]
    pub page_index: i32,
    #[serde(default = "default_page_size")]
    pub page_size: i32,
}
