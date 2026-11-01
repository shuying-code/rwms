use serde::Serialize;
use crate::domain::aggregate::supplier::Supplier;

/// 供应商响应 DTO
#[derive(Debug, Serialize)]
pub struct SupplierResponse {
    pub supplier_id: i32,
    pub supplier_name: String,
    pub contact: Option<String>,
    pub phone: Option<String>,
    pub remark: Option<String>,
}

impl From<Supplier> for SupplierResponse {
    fn from(s: Supplier) -> Self {
        Self {
            supplier_id: s.supplier_id(),
            supplier_name: s.supplier_name().to_string(),
            contact: s.contact().as_ref().map(|v| v.as_str().to_string()),
            phone: s.phone().as_ref().map(|v| v.as_str().to_string()),
            remark: s.remark().clone(),
        }
    }
}
