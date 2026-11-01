use serde::Deserialize;

/// 创建客户请求参数
#[derive(Debug, Deserialize)]
pub struct CreateCustomerRequest {
    pub customer_name: String,
    pub contact: Option<String>,
    pub phone: Option<String>,
    pub remark: Option<String>,
}

/// 更新客户请求参数
#[derive(Debug, Deserialize)]
pub struct UpdateCustomerRequest {
    pub customer_id: i32,
    pub customer_name: String,
    pub contact: Option<String>,
    pub phone: Option<String>,
    pub remark: Option<String>,
}
