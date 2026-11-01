use serde::Serialize;
use sea_orm::entity::prelude::DateTimeUtc;
use rust_decimal::Decimal;

use super::order_output::{OrderDetail, OrderItemWithMaterial};

/// 入库单明细（带物资名称，类型别名）
pub type InOrderItemWithMaterial = OrderItemWithMaterial;

/// 入库单列表项（含供应商名称、仓库名称、操作人用户名）
#[derive(Debug, Serialize)]
pub struct InOrderListItem {
    pub order_id: String,
    pub warehouse_id: String,
    pub warehouse_name: String,
    pub supplier_id: i32,
    pub supplier_name: String,
    pub operator_id: String,
    pub operator: String,
    pub contact: Option<String>,
    pub phone: Option<String>,
    pub order_status: String,
    pub order_type: Option<String>,
    pub purchase_time: Option<DateTimeUtc>,
    pub complete_time: Option<DateTimeUtc>,
    pub remark: Option<String>,
}

/// 入库单详情项（含明细及关联名称）
pub type InOrderDetailItem = OrderDetail<InOrderListItem, InOrderItemWithMaterial>;

/// 入库单明细响应 DTO
#[derive(Debug, Serialize)]
pub struct InOrderItemResponse {
    pub item_id: i32,
    pub order_id: String,
    pub material_id: String,
    pub num: i32,
    pub purchase_price: Decimal,
    pub sell_price: Decimal,
    pub total_amount: Option<Decimal>,
}

impl From<crate::domain::aggregate::in_order_item::InOrderItem> for InOrderItemResponse {
    fn from(i: crate::domain::aggregate::in_order_item::InOrderItem) -> Self {
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
