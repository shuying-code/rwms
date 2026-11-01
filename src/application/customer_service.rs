// ============================================
// 客户应用服务
// 标准 CRUD，无复杂业务逻辑
// ============================================

use std::sync::Arc;
use crate::domain::aggregate::customer::Customer;
use crate::domain::aggregate::contact::Contact;
use crate::domain::aggregate::phone::Phone;
use crate::domain::repository::customer_repo::CustomerRepository;
use crate::application::error::ServiceError;

/// 客户应用服务
#[derive(Clone)]
pub struct CustomerApplicationService {
    repo: Arc<dyn CustomerRepository>,         // 客户仓储（trait object）
}

impl CustomerApplicationService {
    /// 构造函数：接受仓储 trait object（由组合根注入）
    pub fn new(repo: Arc<dyn CustomerRepository>) -> Self {
        Self { repo }
    }

    /// 分页查询客户列表
    pub async fn list_customers_paginated(&self, page: u64, page_size: u64) -> Result<Vec<Customer>, ServiceError> {
        self.repo.list_paginated(page, page_size).await.map_err(ServiceError::from)
    }

    /// 根据ID查询单条客户
    pub async fn get_customer_by_id(&self, customer_id: i32) -> Result<Option<Customer>, ServiceError> {
        self.repo.find_by_id(customer_id).await.map_err(ServiceError::from)
    }

    /// 新建客户
    pub async fn save_customer(&self, customer_name: String, contact: Option<String>, phone: Option<String>, remark: Option<String>) -> Result<(), ServiceError> {
        let contact_vo = contact.map(|s| Contact::new(s).unwrap());
        let phone_vo = phone.map(|s| Phone::new(s).unwrap());
        let customer = Customer::new(customer_name, contact_vo, phone_vo, remark);
        self.repo.create(customer).await.map_err(ServiceError::from)?;
        Ok(())
    }

    /// 更新客户
    pub async fn update_customer(&self, customer_id: i32, customer_name: String, contact: Option<String>, phone: Option<String>, remark: Option<String>) -> Result<(), ServiceError> {
        let existing = self.get_customer_by_id(customer_id).await?;
        match existing {
            Some(mut c) => {
                let contact_vo = contact.map(|s| Contact::new(s).unwrap());
                let phone_vo = phone.map(|s| Phone::new(s).unwrap());
                c.update(customer_name, contact_vo, phone_vo, remark)?;
                self.repo.update(c).await.map_err(ServiceError::from)?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("客户不存在".to_string())),
        }
    }

    /// 删除客户
    pub async fn delete_customer(&self, customer_id: i32) -> Result<(), ServiceError> {
        let existing = self.get_customer_by_id(customer_id).await?;
        match existing {
            Some(_) => {
                self.repo.delete_by_id(customer_id).await.map_err(ServiceError::from)?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("客户不存在".to_string())),
        }
    }

    /// 统计客户总数
    #[allow(dead_code)]
    pub async fn get_total_count(&self) -> Result<u64, ServiceError> {
        self.repo.count().await.map_err(ServiceError::from)
    }
}
