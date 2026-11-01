// ============================================
// 物资应用服务
// CRUD + 分类名称关联查询
// ============================================

use std::sync::Arc;
use crate::domain::aggregate::material::Material;
use crate::domain::aggregate::unit::Unit;
use crate::domain::repository::{material_repo::MaterialRepository, category_repo::CategoryRepository};
use crate::application::error::ServiceError;
use crate::interfaces::dto::input::material_input::{CreateMaterialRequest, UpdateMaterialRequest};
use crate::interfaces::dto::output::material_output::MaterialListItem;

/// 物资应用服务
#[derive(Clone)]
pub struct MaterialApplicationService {
    material_repo: Arc<dyn MaterialRepository>,  // 物资仓储（trait object）
    category_repo: Arc<dyn CategoryRepository>,  // 分类仓储（用于名称关联查询）
}

impl MaterialApplicationService {
    /// 构造函数：接受仓储 trait object（由组合根注入）
    pub fn new(material_repo: Arc<dyn MaterialRepository>, category_repo: Arc<dyn CategoryRepository>) -> Self {
        Self { material_repo, category_repo }
    }

    /// 分页查询物资列表（关联分类名称）
    pub async fn list_materials_with_category(&self, page: u64, page_size: u64) -> Result<Vec<MaterialListItem>, ServiceError> {
        let materials = self.material_repo.list_paginated(page, page_size).await.map_err(ServiceError::from)?;
        let category_ids: Vec<i32> = materials.iter().map(|m| m.category_id()).collect();
        let categories = self.get_category_names_by_ids(&category_ids).await?;

        let items: Vec<MaterialListItem> = materials.into_iter().map(|m| MaterialListItem {
            material_id: m.material_id().to_string(),
            material_name: m.material_name().to_string(),
            category_id: m.category_id(),
            category_name: categories.get(&m.category_id()).cloned().unwrap_or_default(),
            unit: m.unit().to_string(),
            purchase_price: m.purchase_price(),
            sell_price: m.sell_price(),
            total_stock: m.total_stock(),
            create_time: m.create_time().clone(),
            update_time: m.update_time().clone(),
        }).collect();

        Ok(items)
    }

    /// 根据ID查询单条物资（关联分类名称）
    pub async fn get_material_with_category(&self, material_id: &str) -> Result<Option<MaterialListItem>, ServiceError> {
        let m = self.material_repo.find_by_id(material_id).await.map_err(ServiceError::from)?;
        match m {
            Some(m) => {
                let category = self.category_repo.find_by_id(m.category_id()).await.map_err(ServiceError::from)?;
                let category_name = category.map(|c| c.category_name().to_string()).unwrap_or_default();
                Ok(Some(MaterialListItem {
                    material_id: m.material_id().to_string(),
                    material_name: m.material_name().to_string(),
                    category_id: m.category_id(),
                    category_name,
                    unit: m.unit().to_string(),
                    purchase_price: m.purchase_price(),
                    sell_price: m.sell_price(),
                    total_stock: m.total_stock(),
                    create_time: m.create_time().clone(),
                    update_time: m.update_time().clone(),
                }))
            }
            None => Ok(None),
        }
    }

    /// 新建物资
    pub async fn save_material(&self, payload: &CreateMaterialRequest) -> Result<(), ServiceError> {
        let unit = Unit::new(payload.unit.clone()).map_err(|e| ServiceError::Validation(e))?;
        let material = Material::new(
            payload.material_id.clone(),
            payload.material_name.clone(),
            payload.category_id,
            unit,
            payload.purchase_price,
            payload.sell_price,
        );
        self.material_repo.create(material).await.map_err(ServiceError::from)?;
        Ok(())
    }

    /// 更新物资
    pub async fn update_material(&self, payload: &UpdateMaterialRequest) -> Result<(), ServiceError> {
        let material = self.material_repo.find_by_id(&payload.material_id).await.map_err(ServiceError::from)?;
        match material {
            Some(mut m) => {
                let unit = Unit::new(payload.unit.clone()).map_err(|e| ServiceError::Validation(e))?;
                m.update(
                    payload.material_name.clone(),
                    payload.category_id,
                    unit,
                    payload.purchase_price,
                    payload.sell_price,
                )?;
                self.material_repo.update(m).await.map_err(ServiceError::from)?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("物资不存在".to_string())),
        }
    }

    /// 删除物资
    pub async fn delete_material(&self, material_id: &str) -> Result<(), ServiceError> {
        let material = self.material_repo.find_by_id(material_id).await.map_err(ServiceError::from)?;
        match material {
            Some(_) => {
                self.material_repo.delete_by_id(material_id).await.map_err(ServiceError::from)?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("物资不存在".to_string())),
        }
    }

    /// 批量查询分类名称（用于列表关联显示）
    pub(crate) async fn get_category_names_by_ids(&self, ids: &[i32]) -> Result<std::collections::HashMap<i32, String>, ServiceError> {
        let categories = self.category_repo.find_by_ids(ids).await.map_err(ServiceError::from)?;
        Ok(categories.into_iter().map(|c| (c.category_id(), c.category_name().to_string())).collect())
    }
}
