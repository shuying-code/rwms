use serde::Deserialize;
use chrono::{DateTime, Utc};

use super::order_input::OrderItemRequest;

/// 入库单明细项（类型别名）
pub type InOrderItemRequest = OrderItemRequest;

/// 创建入库单请求参数
#[derive(Debug, Deserialize)]
pub struct CreateInOrderRequest {
    pub order_id: String,
    pub warehouse_id: String,
    pub supplier_id: i32,
    pub contact: Option<String>,
    pub phone: Option<String>,
    pub purchase_time: DateTime<Utc>,
    pub remark: Option<String>,
    pub items: Vec<InOrderItemRequest>,
}
