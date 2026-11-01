// ============================================
// 库存应用服务
// 查询 + 名称关联，不直接修改库存（库存变动由入库/出库审核触发）
// ============================================

use std::sync::Arc;
use chrono::{DateTime, Utc};
use crate::domain::aggregate::stock::Stock;
use crate::domain::repository::{stock_repo::StockRepository, material_repo::MaterialRepository, warehouse_repo::WarehouseRepository};
use crate::application::error::ServiceError;
use crate::interfaces::dto::output::stock_output::StockListItem;

/// 库存应用服务
#[derive(Clone)]
pub struct StockApplicationService {
    stock_repo: Arc<dyn StockRepository>,      // 库存仓储（trait object）
    material_repo: Arc<dyn MaterialRepository>, // 物资仓储（用于名称关联）
    warehouse_repo: Arc<dyn WarehouseRepository>, // 仓库仓储（用于名称关联）
}

impl StockApplicationService {
    /// 构造函数：接受仓储 trait object（由组合根注入）
    pub fn new(stock_repo: Arc<dyn StockRepository>, material_repo: Arc<dyn MaterialRepository>, warehouse_repo: Arc<dyn WarehouseRepository>) -> Self {
        Self { stock_repo, material_repo, warehouse_repo }
    }

    /// 条件过滤 + 分页查询库存列表（关联物资名称和仓库名称）
    pub async fn list_stocks_with_names(
        &self,
        material_code: Option<&str>,
        warehouse_id: Option<&str>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<StockListItem>, ServiceError> {
        let stocks = self.stock_repo.list_filtered_paginated(
            material_code, warehouse_id,
            start_date.as_ref(), end_date.as_ref(),
            page, page_size,
        ).await.map_err(ServiceError::from)?;

        let material_ids: Vec<String> = stocks.iter().map(|s| s.material_id().to_string()).collect();
        let materials = self.get_material_names_by_ids(&material_ids).await?;

        let warehouse_ids: Vec<String> = stocks.iter().map(|s| s.warehouse_id().to_string()).collect();
        let warehouses = self.get_warehouse_names_by_ids(&warehouse_ids).await?;

        let items: Vec<StockListItem> = stocks.into_iter().map(|s| {
            let stock_num = s.stock_num();
            let update_time = s.update_time().clone();
            let in_time = s.in_time().clone();
            StockListItem {
            stock_id: s.stock_id(),
            material_id: s.material_id().to_string(),
            material_name: materials.get(s.material_id()).cloned().unwrap_or_default(),
            warehouse_id: s.warehouse_id().to_string(),
            warehouse_name: warehouses.get(s.warehouse_id()).cloned().unwrap_or_default(),
            stock_num,
            in_time,
            update_time,
            }
        }).collect();

        Ok(items)
    }

    /// 根据物资+仓库查询库存
    #[allow(dead_code)]
    pub async fn get_stock_by_material_and_warehouse(&self, material_id: &str, warehouse_id: &str) -> Result<Option<Stock>, ServiceError> {
        self.stock_repo.find_by_material_and_warehouse(material_id, warehouse_id).await.map_err(ServiceError::from)
    }

    /// 批量查询物资名称（用于列表关联显示）
    pub(crate) async fn get_material_names_by_ids(&self, ids: &[String]) -> Result<std::collections::HashMap<String, String>, ServiceError> {
        let materials = self.material_repo.find_by_ids(ids).await.map_err(ServiceError::from)?;
        Ok(materials.into_iter().map(|m| (m.material_id().to_string(), m.material_name().to_string())).collect())
    }

    /// 批量查询仓库名称（用于列表关联显示）
    pub(crate) async fn get_warehouse_names_by_ids(&self, ids: &[String]) -> Result<std::collections::HashMap<String, String>, ServiceError> {
        let warehouses = self.warehouse_repo.find_by_ids(ids).await.map_err(ServiceError::from)?;
        Ok(warehouses.into_iter().map(|w| (w.warehouse_id().to_string(), w.warehouse_code().to_string())).collect())
    }
}
