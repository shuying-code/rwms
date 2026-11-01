use serde::Deserialize;
use rust_decimal::Decimal;

/// 创建物资请求参数
#[derive(Debug, Deserialize)]
pub struct CreateMaterialRequest {
    pub material_id: String,
    pub material_name: String,
    pub category_id: i32,
    pub unit: String,
    pub purchase_price: Decimal,
    pub sell_price: Decimal,
}

/// 更新物资请求参数
#[derive(Debug, Deserialize)]
pub struct UpdateMaterialRequest {
    pub material_id: String,
    pub material_name: String,
    pub category_id: i32,
    pub unit: String,
    pub purchase_price: Decimal,
    pub sell_price: Decimal,
}
