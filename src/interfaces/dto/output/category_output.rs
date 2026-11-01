use serde::Serialize;
use crate::domain::aggregate::material_category::MaterialCategory;

/// 物资分类响应 DTO
#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub category_id: i32,
    pub category_name: String,
    pub remark: Option<String>,
}

impl From<MaterialCategory> for CategoryResponse {
    fn from(c: MaterialCategory) -> Self {
        Self {
            category_id: c.category_id(),
            category_name: c.category_name().to_string(),
            remark: c.remark().clone(),
        }
    }
}
