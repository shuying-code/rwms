use serde::Deserialize;

/// 创建分类请求参数
#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub category_name: String,
    pub remark: Option<String>,
}

/// 更新分类请求参数
#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub category_id: i32,
    pub category_name: String,
    pub remark: Option<String>,
}
