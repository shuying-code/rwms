// ============================================
// 仓库应用服务
// 标准 CRUD，无复杂业务逻辑
// ============================================

use std::sync::Arc;
use crate::domain::aggregate::warehouse::Warehouse;
use crate::domain::aggregate::warehouse_code::WarehouseCode;
use crate::domain::repository::warehouse_repo::WarehouseRepository;
use crate::application::error::ServiceError;

/// 仓库应用服务
#[derive(Clone)]
pub struct WarehouseApplicationService {
    repo: Arc<dyn WarehouseRepository>,        // 仓库仓储（trait object）
}

impl WarehouseApplicationService {
    /// 构造函数：接受仓储 trait object（由组合根注入）
    pub fn new(repo: Arc<dyn WarehouseRepository>) -> Self {
        Self { repo }
    }

    /// 分页查询仓库列表
    pub async fn list_warehouses_paginated(&self, page: u64, page_size: u64) -> Result<Vec<Warehouse>, ServiceError> {
        self.repo.list_paginated(page, page_size).await.map_err(ServiceError::from)
    }

    /// 根据ID查询单条仓库
    pub async fn get_warehouse_by_id(&self, warehouse_id: &str) -> Result<Option<Warehouse>, ServiceError> {
        self.repo.find_by_id(warehouse_id).await.map_err(ServiceError::from)
    }

    /// 新建仓库
    pub async fn save_warehouse(&self, warehouse_id: String, warehouse_code: String, manager: Option<String>, max_space: i32, _remain_space: i32) -> Result<(), ServiceError> {
        let warehouse_code_vo = WarehouseCode::new(warehouse_code).map_err(|e| ServiceError::Validation(e))?;
        // Warehouse::new 自动设置 remain_space = max_space, create_time = now
        let warehouse = Warehouse::new(warehouse_id, warehouse_code_vo, manager, max_space)?;
        self.repo.create(warehouse).await.map_err(ServiceError::from)?;
        Ok(())
    }

    /// 更新仓库
    pub async fn update_warehouse(&self, warehouse_id: &str, warehouse_code: String, manager: Option<String>, max_space: i32) -> Result<(), ServiceError> {
        let existing = self.get_warehouse_by_id(warehouse_id).await?;
        match existing {
            Some(mut w) => {
                let warehouse_code_vo = WarehouseCode::new(warehouse_code).map_err(|e| ServiceError::Validation(e))?;
                w.update_info(warehouse_code_vo, manager)?;
                w.adjust_capacity(max_space)?;
                self.repo.update(w).await.map_err(ServiceError::from)?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("仓库不存在".to_string())),
        }
    }

    /// 删除仓库
    pub async fn delete_warehouse(&self, warehouse_id: &str) -> Result<(), ServiceError> {
        let existing = self.get_warehouse_by_id(warehouse_id).await?;
        match existing {
            Some(_) => {
                self.repo.delete_by_id(warehouse_id).await.map_err(ServiceError::from)?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("仓库不存在".to_string())),
        }
    }

    /// 统计仓库总数
    #[allow(dead_code)]
    pub async fn get_total_count(&self) -> Result<u64, ServiceError> {
        self.repo.count().await.map_err(ServiceError::from)
    }
}
