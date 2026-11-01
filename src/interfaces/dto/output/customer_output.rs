use serde::Serialize;
use crate::domain::aggregate::customer::Customer;

/// 客户响应 DTO
#[derive(Debug, Serialize)]
pub struct CustomerResponse {
    pub customer_id: i32,
    pub customer_name: String,
    pub contact: Option<String>,
    pub phone: Option<String>,
    pub remark: Option<String>,
}

impl From<Customer> for CustomerResponse {
    fn from(c: Customer) -> Self {
        Self {
            customer_id: c.customer_id(),
            customer_name: c.customer_name().to_string(),
            contact: c.contact().as_ref().map(|v| v.as_str().to_string()),
            phone: c.phone().as_ref().map(|v| v.as_str().to_string()),
            remark: c.remark().clone(),
        }
    }
}
