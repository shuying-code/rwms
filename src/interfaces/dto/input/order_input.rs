use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

/// 订单明细项（入库/出库通用）
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderItemRequest {
    pub material_id: String,
    pub num: i32,
    pub purchase_price: Decimal,
    pub sell_price: Decimal,
}

/// 审核订单请求参数
#[derive(Debug, Deserialize)]
pub struct AuditOrderRequest {
    pub order_id: String,
    pub order_status: String,
}
