use serde::Serialize;
use rust_decimal::Decimal;

/// 订单详情响应（含嵌入的明细列表，入库/出库通用）
#[derive(Debug, Serialize)]
pub struct OrderDetail<T, I> {
    #[serde(flatten)]
    pub order: T,
    pub items: Vec<I>,
}

/// 订单明细（带物资名称，入库/出库通用）
#[derive(Debug, Serialize)]
pub struct OrderItemWithMaterial {
    pub item_id: i32,
    pub order_id: String,
    pub material_id: String,
    pub material_name: String,
    pub num: i32,
    pub purchase_price: Decimal,
    pub sell_price: Decimal,
    pub total_amount: Option<Decimal>,
}
