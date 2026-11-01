// ============================================
// 物资分类应用服务
// 标准 CRUD，无复杂业务逻辑
// ============================================

use std::sync::Arc;
use crate::domain::aggregate::material_category::MaterialCategory;
use crate::domain::repository::category_repo::CategoryRepository;
use crate::application::error::ServiceError;

/// 物资分类应用服务
#[derive(Clone)]
pub struct CategoryApplicationService {
    repo: Arc<dyn CategoryRepository>,         // 分类仓储（trait object）
}

impl CategoryApplicationService {
    /// 构造函数：接受仓储 trait object（由组合根注入）
    pub fn new(repo: Arc<dyn CategoryRepository>) -> Self {
        Self { repo }
    }

    /// 分页查询分类列表
    pub async fn list_categories_paginated(&self, page: u64, page_size: u64) -> Result<Vec<MaterialCategory>, ServiceError> {
        self.repo.list_paginated(page, page_size).await.map_err(ServiceError::from)
    }

    /// 根据ID查询单条分类
    pub async fn get_category_by_id(&self, category_id: i32) -> Result<Option<MaterialCategory>, ServiceError> {
        self.repo.find_by_id(category_id).await.map_err(ServiceError::from)
    }

    /// 新建分类
    pub async fn save_category(&self, category_name: String, remark: Option<String>) -> Result<(), ServiceError> {
        let category = MaterialCategory::new(category_name, remark);
        self.repo.create(category).await.map_err(ServiceError::from)?;
        Ok(())
    }

    /// 更新分类
    pub async fn update_category(&self, category_id: i32, category_name: String, remark: Option<String>) -> Result<(), ServiceError> {
        let existing = self.get_category_by_id(category_id).await?;
        match existing {
            Some(mut c) => {
                c.update(category_name, remark)?;
                self.repo.update(c).await.map_err(ServiceError::from)?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("分类不存在".to_string())),
        }
    }

    /// 删除分类
    pub async fn delete_category(&self, category_id: i32) -> Result<(), ServiceError> {
        let existing = self.get_category_by_id(category_id).await?;
        match existing {
            Some(_) => {
                self.repo.delete_by_id(category_id).await.map_err(ServiceError::from)?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("分类不存在".to_string())),
        }
    }

    /// 统计分类总数
    #[allow(dead_code)]
    pub async fn get_total_count(&self) -> Result<u64, ServiceError> {
        self.repo.count().await.map_err(ServiceError::from)
    }
}
