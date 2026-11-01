// ============================================
// 供应商应用服务
// 标准 CRUD，无复杂业务逻辑
// ============================================

use std::sync::Arc;
use crate::domain::aggregate::supplier::Supplier;
use crate::domain::aggregate::contact::Contact;
use crate::domain::aggregate::phone::Phone;
use crate::domain::repository::supplier_repo::SupplierRepository;
use crate::application::error::ServiceError;

/// 供应商应用服务
#[derive(Clone)]
pub struct SupplierApplicationService {
    repo: Arc<dyn SupplierRepository>,         // 供应商仓储（trait object）
}

impl SupplierApplicationService {
    /// 构造函数：接受仓储 trait object（由组合根注入）
    pub fn new(repo: Arc<dyn SupplierRepository>) -> Self {
        Self { repo }
    }

    /// 分页查询供应商列表
    pub async fn list_suppliers_paginated(&self, page: u64, page_size: u64) -> Result<Vec<Supplier>, ServiceError> {
        self.repo.list_paginated(page, page_size).await.map_err(ServiceError::from)
    }

    /// 根据ID查询单条供应商
    pub async fn get_supplier_by_id(&self, supplier_id: i32) -> Result<Option<Supplier>, ServiceError> {
        self.repo.find_by_id(supplier_id).await.map_err(ServiceError::from)
    }

    /// 新建供应商
    pub async fn save_supplier(&self, supplier_name: String, contact: Option<String>, phone: Option<String>, remark: Option<String>) -> Result<(), ServiceError> {
        let contact_vo = contact.map(|s| Contact::new(s).unwrap());
        let phone_vo = phone.map(|s| Phone::new(s).unwrap());
        let supplier = Supplier::new(supplier_name, contact_vo, phone_vo, remark);
        self.repo.create(supplier).await.map_err(ServiceError::from)?;
        Ok(())
    }

    /// 更新供应商
    pub async fn update_supplier(&self, supplier_id: i32, supplier_name: String, contact: Option<String>, phone: Option<String>, remark: Option<String>) -> Result<(), ServiceError> {
        let existing = self.get_supplier_by_id(supplier_id).await?;
        match existing {
            Some(mut s) => {
                let contact_vo = contact.map(|s| Contact::new(s).unwrap());
                let phone_vo = phone.map(|s| Phone::new(s).unwrap());
                s.update(supplier_name, contact_vo, phone_vo, remark)?;
                self.repo.update(s).await.map_err(ServiceError::from)?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("供应商不存在".to_string())),
        }
    }

    /// 删除供应商
    pub async fn delete_supplier(&self, supplier_id: i32) -> Result<(), ServiceError> {
        let existing = self.get_supplier_by_id(supplier_id).await?;
        match existing {
            Some(_) => {
                self.repo.delete_by_id(supplier_id).await.map_err(ServiceError::from)?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("供应商不存在".to_string())),
        }
    }

    /// 统计供应商总数
    #[allow(dead_code)]
    pub async fn get_total_count(&self) -> Result<u64, ServiceError> {
        self.repo.count().await.map_err(ServiceError::from)
    }
}
