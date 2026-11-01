// ============================================
// 入库单应用服务
// 使用领域实体 approve/reject、InOrderDomainService
// ============================================

use std::sync::Arc;
use crate::domain::aggregate::in_order::InOrder;
use crate::domain::aggregate::in_order_item::InOrderItem;
use crate::domain::aggregate::order_status::OrderStatus;
use crate::domain::aggregate::contact::Contact;
use crate::domain::aggregate::phone::Phone;
use crate::domain::domain_service::in_order_domain_service::{InOrderDomainService, StockChange};
use crate::domain::repository::{
    in_order_repo::InOrderRepository, in_order_item_repo::InOrderItemRepository,
    stock_repo::StockRepository, supplier_repo::SupplierRepository,
    warehouse_repo::WarehouseRepository, user_repo::UserRepository,
    material_repo::MaterialRepository, TransactionFactory,
};
use crate::application::error::ServiceError;
use crate::interfaces::dto::input::in_order_input::CreateInOrderRequest;
use crate::interfaces::dto::input::order_input::AuditOrderRequest;
use crate::interfaces::dto::output::in_order_output::{InOrderListItem, InOrderDetailItem, InOrderItemWithMaterial};

/// 入库单应用服务（事务编排 + 名称关联查询）
#[derive(Clone)]
pub struct InOrderApplicationService {
    transaction_factory: Arc<dyn TransactionFactory>,      // 事务工厂（Domain 层抽象）
    in_order_repo: Arc<dyn InOrderRepository>,           // 入库单仓储
    in_order_item_repo: Arc<dyn InOrderItemRepository>,   // 入库单明细仓储
    stock_repo: Arc<dyn StockRepository>,                 // 库存仓储
    supplier_repo: Arc<dyn SupplierRepository>,           // 供应商仓储（名称关联）
    warehouse_repo: Arc<dyn WarehouseRepository>,         // 仓库仓储（名称关联）
    user_repo: Arc<dyn UserRepository>,                   // 用户仓储（名称关联）
    material_repo: Arc<dyn MaterialRepository>,           // 物资仓储（名称关联）
}

impl InOrderApplicationService {
    /// 构造函数：接受仓储和事务工厂 trait object（由组合根注入）
    pub fn new(
        transaction_factory: Arc<dyn TransactionFactory>,
        in_order_repo: Arc<dyn InOrderRepository>,
        in_order_item_repo: Arc<dyn InOrderItemRepository>,
        stock_repo: Arc<dyn StockRepository>,
        supplier_repo: Arc<dyn SupplierRepository>,
        warehouse_repo: Arc<dyn WarehouseRepository>,
        user_repo: Arc<dyn UserRepository>,
        material_repo: Arc<dyn MaterialRepository>,
    ) -> Self {
        Self { transaction_factory, in_order_repo, in_order_item_repo, stock_repo, supplier_repo, warehouse_repo, user_repo, material_repo }
    }

    /// 分页查询入库单列表（关联供应商/仓库/操作员名称）
    pub async fn list_in_orders_with_names(&self, page: u64, page_size: u64) -> Result<Vec<InOrderListItem>, ServiceError> {
        let orders = self.in_order_repo.list_paginated(page, page_size).await.map_err(ServiceError::from)?;

        let supplier_ids: Vec<i32> = orders.iter().map(|o| o.supplier_id()).collect();
        let warehouse_ids: Vec<String> = orders.iter().map(|o| o.warehouse_id().to_string()).collect();
        let operator_ids: Vec<String> = orders.iter().map(|o| o.operator_id().to_string()).collect();

        let suppliers = self.get_supplier_names_by_ids(&supplier_ids).await?;
        let warehouses = self.get_warehouse_names_by_ids(&warehouse_ids).await?;
        let operators = self.get_operator_names_by_ids(&operator_ids).await?;

        let items: Vec<InOrderListItem> = orders.into_iter().map(|o| {
            let order_status = o.order_status().to_string();
            let complete_time = o.complete_time().clone();
            InOrderListItem {
            warehouse_name: warehouses.get(o.warehouse_id()).cloned().unwrap_or_default(),
            supplier_name: suppliers.get(&o.supplier_id()).cloned().unwrap_or_default(),
            operator: operators.get(o.operator_id()).cloned().unwrap_or_default(),
            order_id: o.order_id().to_string(),
            warehouse_id: o.warehouse_id().to_string(),
            supplier_id: o.supplier_id(),
            operator_id: o.operator_id().to_string(),
            contact: o.contact().as_ref().map(|c| c.as_str().to_string()),
            phone: o.phone().as_ref().map(|p| p.as_str().to_string()),
            order_status,
            order_type: Some(o.order_type().as_str().to_string()),
            purchase_time: o.purchase_time().clone(),
            complete_time,
            remark: o.remark().clone(),
            }
        }).collect();

        Ok(items)
    }

    /// 查询入库单详情（关联所有名称）
    pub async fn get_in_order_detail(&self, order_id: &str) -> Result<Option<InOrderDetailItem>, ServiceError> {
        let order = self.in_order_repo.find_by_id(order_id).await.map_err(ServiceError::from)?;
        match order {
            Some(o) => {
                let items = self.in_order_item_repo.find_by_order_id(order_id).await.map_err(ServiceError::from)?;
                let material_ids: Vec<String> = items.iter().map(|i| i.material_id().to_string()).collect();
                let materials = self.get_material_names_by_ids(&material_ids).await?;

                let enriched_items: Vec<InOrderItemWithMaterial> = items.into_iter().map(|i| InOrderItemWithMaterial {
                    material_name: materials.get(i.material_id()).cloned().unwrap_or_default(),
                    item_id: i.item_id(),
                    order_id: i.order_id().to_string(),
                    material_id: i.material_id().to_string(),
                    num: i.num(),
                    purchase_price: i.purchase_price(),
                    sell_price: i.sell_price(),
                    total_amount: i.total_amount(),
                }).collect();

                let supplier_name = self.supplier_repo.find_by_id(o.supplier_id()).await.map_err(ServiceError::from)?.map(|s| s.supplier_name().to_string());
                let warehouse_name = self.warehouse_repo.find_by_id(o.warehouse_id()).await.map_err(ServiceError::from)?.map(|w| w.warehouse_code().to_string());
                let operator_name = self.user_repo.find_by_id(o.operator_id()).await.map_err(ServiceError::from)?.map(|u| u.username().to_string());

                let order_status = o.order_status().to_string();
                let complete_time = o.complete_time().clone();
                let order_info = InOrderListItem {
                    order_id: o.order_id().to_string(),
                    warehouse_id: o.warehouse_id().to_string(),
                    warehouse_name: warehouse_name.unwrap_or_default(),
                    supplier_id: o.supplier_id(),
                    supplier_name: supplier_name.unwrap_or_default(),
                    operator_id: o.operator_id().to_string(),
                    operator: operator_name.unwrap_or_default(),
                    contact: o.contact().as_ref().map(|c| c.as_str().to_string()),
                    phone: o.phone().as_ref().map(|p| p.as_str().to_string()),
                    order_status,
                    order_type: Some(o.order_type().as_str().to_string()),
                    purchase_time: o.purchase_time().clone(),
                    complete_time,
                    remark: o.remark().clone(),
                };

                Ok(Some(InOrderDetailItem { order: order_info, items: enriched_items }))
            }
            None => Ok(None),
        }
    }

    /// 创建入库单（事务：创建订单 + 创建明细）
    pub async fn create_in_order(&self, operator_id: &str, payload: &CreateInOrderRequest) -> Result<(), ServiceError> {
        let existing = self.in_order_repo.find_by_id(&payload.order_id).await.map_err(ServiceError::from)?;
        if existing.is_some() {
            return Err(ServiceError::AlreadyExists(format!("入库单号【{}】已存在", payload.order_id)));
        }

        let contact_vo = payload.contact.clone().map(|s| Contact::new(s).unwrap());
        let phone_vo = payload.phone.clone().map(|s| Phone::new(s).unwrap());
        let mut order = InOrder::new(
            payload.order_id.clone(),
            payload.warehouse_id.clone(),
            payload.supplier_id,
            operator_id.to_string(),
            contact_vo,
            phone_vo,
            Some(payload.purchase_time),
            payload.remark.clone(),
        );

        for item in &payload.items {
            let order_item = InOrderItem::new(
                item.material_id.clone(),
                item.num,
                item.purchase_price,
                item.sell_price,
            )?;
            order.add_item(order_item).map_err(|e| ServiceError::Validation(e.to_string()))?;
        }

        InOrderDomainService::validate_in_order_items(order.get_items()).map_err(ServiceError::from)?;

        let txn = self.transaction_factory.begin_transaction().await.map_err(|e| ServiceError::Database(e))?;

        self.in_order_repo.create_in_txn(&txn, order.clone()).await.map_err(ServiceError::from)?;

        for item in order.get_items() {
            self.in_order_item_repo.create_in_txn(&txn, item.clone()).await.map_err(ServiceError::from)?;
        }

        txn.commit().await.map_err(|e| ServiceError::Database(e))?;
        Ok(())
    }

    /// 审核入库单（使用 InOrderDomainService 编排跨实体业务规则）
    pub async fn audit_in_order(&self, payload: &AuditOrderRequest) -> Result<(), ServiceError> {
        let mut order = self.in_order_repo.find_by_id(&payload.order_id).await.map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound("入库单不存在".to_string()))?;

        if OrderStatus::from_str(&payload.order_status) == Some(OrderStatus::Approved) {
            let txn = self.transaction_factory.begin_transaction().await.map_err(|e| ServiceError::Database(e))?;

            let items = self.in_order_item_repo.find_by_order_id_in_txn(&txn, &payload.order_id).await.map_err(ServiceError::from)?;

            let mut existing_stocks = Vec::with_capacity(items.len());
            for item in &items {
                let stock = self.stock_repo.find_by_material_and_warehouse_in_txn(&txn, item.material_id(), order.warehouse_id()).await.map_err(ServiceError::from)?;
                existing_stocks.push(stock);
            }

            let changes = InOrderDomainService::process_inbound_approval(&mut order, &items, existing_stocks)
                .map_err(ServiceError::from)?;

            for change in changes {
                match change {
                    StockChange::CreateNew(new_stock) => {
                        self.stock_repo.create_in_txn(&txn, new_stock).await.map_err(ServiceError::from)?;
                    }
                    StockChange::UpdateExisting(updated_stock) => {
                        self.stock_repo.update_in_txn(&txn, updated_stock).await.map_err(ServiceError::from)?;
                    }
                }
            }

            self.in_order_repo.update_in_txn(&txn, order).await.map_err(ServiceError::from)?;
            txn.commit().await.map_err(|e| ServiceError::Database(e))?;
        } else {
            order.reject().map_err(|e| ServiceError::Validation(e.to_string()))?;
            self.in_order_repo.update(order).await.map_err(ServiceError::from)?;
        }

        Ok(())
    }

    /// 删除入库单及其明细（事务：先删明细再删订单）
    pub async fn delete_in_order_with_items(&self, order_id: &str) -> Result<(), ServiceError> {
        let order = self.in_order_repo.find_by_id(order_id).await.map_err(ServiceError::from)?;
        match order {
            Some(_) => {
                let txn = self.transaction_factory.begin_transaction().await.map_err(|e| ServiceError::Database(e))?;
                self.in_order_item_repo.delete_by_order_id_in_txn(&txn, order_id).await.map_err(ServiceError::from)?;
                self.in_order_repo.delete_by_id_in_txn(&txn, order_id).await.map_err(ServiceError::from)?;
                txn.commit().await.map_err(|e| ServiceError::Database(e))?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("入库单不存在".to_string())),
        }
    }

    /// 查询入库单的所有明细
    pub async fn list_in_order_items(&self, order_id: &str) -> Result<Vec<InOrderItem>, ServiceError> {
        self.in_order_item_repo.find_by_order_id(order_id).await.map_err(ServiceError::from)
    }

    // --- 批量名称查询辅助方法 ---
    async fn get_supplier_names_by_ids(&self, ids: &[i32]) -> Result<std::collections::HashMap<i32, String>, ServiceError> {
        let list = self.supplier_repo.find_by_ids(ids).await.map_err(ServiceError::from)?;
        Ok(list.into_iter().map(|s| (s.supplier_id(), s.supplier_name().to_string())).collect())
    }
    async fn get_warehouse_names_by_ids(&self, ids: &[String]) -> Result<std::collections::HashMap<String, String>, ServiceError> {
        let list = self.warehouse_repo.find_by_ids(ids).await.map_err(ServiceError::from)?;
        Ok(list.into_iter().map(|w| (w.warehouse_id().to_string(), w.warehouse_code().to_string())).collect())
    }
    async fn get_operator_names_by_ids(&self, ids: &[String]) -> Result<std::collections::HashMap<String, String>, ServiceError> {
        let list = self.user_repo.find_by_ids(ids).await.map_err(ServiceError::from)?;
        Ok(list.into_iter().map(|u| (u.user_id().to_string(), u.username().to_string())).collect())
    }
    async fn get_material_names_by_ids(&self, ids: &[String]) -> Result<std::collections::HashMap<String, String>, ServiceError> {
        let list = self.material_repo.find_by_ids(ids).await.map_err(ServiceError::from)?;
        Ok(list.into_iter().map(|m| (m.material_id().to_string(), m.material_name().to_string())).collect())
    }
}
