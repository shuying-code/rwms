use serde::Serialize;
use sea_orm::entity::prelude::DateTimeUtc;
use rust_decimal::Decimal;

use super::order_output::{OrderDetail, OrderItemWithMaterial};

/// 出库单明细（带物资名称，类型别名）
pub type OutOrderItemWithMaterial = OrderItemWithMaterial;

/// 出库单列表项（含客户名称、仓库名称、操作人用户名）
#[derive(Debug, Serialize)]
pub struct OutOrderListItem {
    pub order_id: String,
    pub warehouse_id: String,
    pub warehouse_name: String,
    pub customer_id: i32,
    pub customer_name: String,
    pub operator_id: String,
    pub operator: String,
    pub contact: Option<String>,
    pub phone: Option<String>,
    pub order_status: String,
    pub order_type: Option<String>,
    pub out_time: Option<DateTimeUtc>,
    pub complete_time: Option<DateTimeUtc>,
    pub remark: Option<String>,
}

/// 出库单详情项（含明细及关联名称）
pub type OutOrderDetailItem = OrderDetail<OutOrderListItem, OutOrderItemWithMaterial>;

/// 出库单明细响应 DTO
#[derive(Debug, Serialize)]
pub struct OutOrderItemResponse {
    pub item_id: i32,
    pub order_id: String,
    pub material_id: String,
    pub num: i32,
    pub purchase_price: Decimal,
    pub sell_price: Decimal,
    pub total_amount: Option<Decimal>,
}

impl From<crate::domain::aggregate::out_order_item::OutOrderItem> for OutOrderItemResponse {
    fn from(i: crate::domain::aggregate::out_order_item::OutOrderItem) -> Self {
        Self {
            item_id: i.item_id(),
            order_id: i.order_id().to_string(),
            material_id: i.material_id().to_string(),
            num: i.num(),
            purchase_price: i.purchase_price(),
            sell_price: i.sell_price(),
            total_amount: i.total_amount(),
        }
    }
}
