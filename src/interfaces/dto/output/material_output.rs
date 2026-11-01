use serde::Serialize;
use rust_decimal::Decimal;
use sea_orm::entity::prelude::DateTimeUtc;

/// 物资列表项（含分类名称）
#[derive(Debug, Serialize)]
pub struct MaterialListItem {
    pub material_id: String,
    pub material_name: String,
    pub category_id: i32,
    pub category_name: String,
    pub unit: String,
    pub purchase_price: Decimal,
    pub sell_price: Decimal,
    pub total_stock: Option<i32>,
    pub create_time: Option<DateTimeUtc>,
    pub update_time: Option<DateTimeUtc>,
}
