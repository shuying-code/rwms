use sea_orm::entity::prelude::DateTimeUtc;

/// 库存列表项（含物资名称、仓库名称）
#[derive(Debug, serde::Serialize)]
pub struct StockListItem {
    pub stock_id: i32,
    pub material_id: String,
    pub material_name: String,
    pub warehouse_id: String,
    pub warehouse_name: String,
    pub stock_num: i32,
    pub in_time: Option<DateTimeUtc>,
    pub update_time: Option<DateTimeUtc>,
}
