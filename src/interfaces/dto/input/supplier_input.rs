use serde::Deserialize;

/// 创建供应商请求参数
#[derive(Debug, Deserialize)]
pub struct CreateSupplierRequest {
    pub supplier_name: String,
    pub contact: Option<String>,
    pub phone: Option<String>,
    pub remark: Option<String>,
}

/// 更新供应商请求参数
#[derive(Debug, Deserialize)]
pub struct UpdateSupplierRequest {
    pub supplier_id: i32,
    pub supplier_name: String,
    pub contact: Option<String>,
    pub phone: Option<String>,
    pub remark: Option<String>,
}
