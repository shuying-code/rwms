use serde::Deserialize;
use chrono::{DateTime, Utc};

use super::order_input::OrderItemRequest;

/// 出库单明细项（类型别名）
pub type OutOrderItemRequest = OrderItemRequest;

/// 创建出库单请求参数
#[derive(Debug, Deserialize)]
pub struct CreateOutOrderRequest {
    pub order_id: String,
    pub warehouse_id: String,
    pub customer_id: i32,
    pub contact: Option<String>,
    pub phone: Option<String>,
    pub out_time: DateTime<Utc>,
    pub remark: Option<String>,
    pub items: Vec<OutOrderItemRequest>,
}
