// ============================================
// 出库单应用服务
// 使用领域实体 approve/reject、OutOrderDomainService
// ============================================

use std::sync::Arc;
use crate::domain::aggregate::out_order::OutOrder;
use crate::domain::aggregate::out_order_item::OutOrderItem;
use crate::domain::aggregate::order_status::OrderStatus;
use crate::domain::aggregate::contact::Contact;
use crate::domain::aggregate::phone::Phone;
use crate::domain::domain_service::out_order_domain_service::OutOrderDomainService;
use crate::domain::repository::{
    out_order_repo::OutOrderRepository, out_order_item_repo::OutOrderItemRepository,
    stock_repo::StockRepository, customer_repo::CustomerRepository,
    warehouse_repo::WarehouseRepository, user_repo::UserRepository,
    material_repo::MaterialRepository, TransactionFactory,
};
use crate::application::error::ServiceError;
use crate::interfaces::dto::input::order_input::AuditOrderRequest;
use crate::interfaces::dto::input::out_order_input::CreateOutOrderRequest;
use crate::interfaces::dto::output::out_order_output::{OutOrderListItem, OutOrderDetailItem, OutOrderItemWithMaterial};

/// 出库单应用服务（事务编排 + 名称关联查询）
#[derive(Clone)]
pub struct OutOrderApplicationService {
    transaction_factory: Arc<dyn TransactionFactory>,      // 事务工厂（Domain 层抽象）
    out_order_repo: Arc<dyn OutOrderRepository>,          // 出库单仓储
    out_order_item_repo: Arc<dyn OutOrderItemRepository>,  // 出库单明细仓储
    stock_repo: Arc<dyn StockRepository>,                  // 库存仓储
    customer_repo: Arc<dyn CustomerRepository>,            // 客户仓储（名称关联）
    warehouse_repo: Arc<dyn WarehouseRepository>,          // 仓库仓储（名称关联）
    user_repo: Arc<dyn UserRepository>,                    // 用户仓储（名称关联）
    material_repo: Arc<dyn MaterialRepository>,            // 物资仓储（名称关联）
}

impl OutOrderApplicationService {
    /// 构造函数：接受仓储和事务工厂 trait object（由组合根注入）
    pub fn new(
        transaction_factory: Arc<dyn TransactionFactory>,
        out_order_repo: Arc<dyn OutOrderRepository>,
        out_order_item_repo: Arc<dyn OutOrderItemRepository>,
        stock_repo: Arc<dyn StockRepository>,
        customer_repo: Arc<dyn CustomerRepository>,
        warehouse_repo: Arc<dyn WarehouseRepository>,
        user_repo: Arc<dyn UserRepository>,
        material_repo: Arc<dyn MaterialRepository>,
    ) -> Self {
        Self { transaction_factory, out_order_repo, out_order_item_repo, stock_repo, customer_repo, warehouse_repo, user_repo, material_repo }
    }

    /// 分页查询出库单列表（关联客户/仓库/操作员名称）
    pub async fn list_out_orders_with_names(&self, page: u64, page_size: u64) -> Result<Vec<OutOrderListItem>, ServiceError> {
        let orders = self.out_order_repo.list_paginated(page, page_size).await.map_err(ServiceError::from)?;

        let customer_ids: Vec<i32> = orders.iter().map(|o| o.customer_id()).collect();
        let warehouse_ids: Vec<String> = orders.iter().map(|o| o.warehouse_id().to_string()).collect();
        let operator_ids: Vec<String> = orders.iter().map(|o| o.operator_id().to_string()).collect();

        let customers = self.get_customer_names_by_ids(&customer_ids).await?;
        let warehouses = self.get_warehouse_names_by_ids(&warehouse_ids).await?;
        let operators = self.get_operator_names_by_ids(&operator_ids).await?;

        let items: Vec<OutOrderListItem> = orders.into_iter().map(|o| {
            let order_status = o.order_status().to_string();
            let complete_time = o.complete_time().clone();
            OutOrderListItem {
            warehouse_name: warehouses.get(o.warehouse_id()).cloned().unwrap_or_default(),
            customer_name: customers.get(&o.customer_id()).cloned().unwrap_or_default(),
            operator: operators.get(o.operator_id()).cloned().unwrap_or_default(),
            order_id: o.order_id().to_string(),
            warehouse_id: o.warehouse_id().to_string(),
            customer_id: o.customer_id(),
            operator_id: o.operator_id().to_string(),
            contact: o.contact().as_ref().map(|c| c.as_str().to_string()),
            phone: o.phone().as_ref().map(|p| p.as_str().to_string()),
            order_status,
            order_type: Some(o.order_type().as_str().to_string()),
            out_time: o.out_time().clone(),
            complete_time,
            remark: o.remark().clone(),
            }
        }).collect();

        Ok(items)
    }

    /// 查询出库单详情（关联所有名称）
    pub async fn get_out_order_detail(&self, order_id: &str) -> Result<Option<OutOrderDetailItem>, ServiceError> {
        let order = self.out_order_repo.find_by_id(order_id).await.map_err(ServiceError::from)?;
        match order {
            Some(o) => {
                let items = self.out_order_item_repo.find_by_order_id(order_id).await.map_err(ServiceError::from)?;
                let material_ids: Vec<String> = items.iter().map(|i| i.material_id().to_string()).collect();
                let materials = self.get_material_names_by_ids(&material_ids).await?;

                let enriched_items: Vec<OutOrderItemWithMaterial> = items.into_iter().map(|i| OutOrderItemWithMaterial {
                    material_name: materials.get(i.material_id()).cloned().unwrap_or_default(),
                    item_id: i.item_id(),
                    order_id: i.order_id().to_string(),
                    material_id: i.material_id().to_string(),
                    num: i.num(),
                    purchase_price: i.purchase_price(),
                    sell_price: i.sell_price(),
                    total_amount: i.total_amount(),
                }).collect();

                let customer_name = self.customer_repo.find_by_id(o.customer_id()).await.map_err(ServiceError::from)?.map(|c| c.customer_name().to_string());
                let warehouse_name = self.warehouse_repo.find_by_id(o.warehouse_id()).await.map_err(ServiceError::from)?.map(|w| w.warehouse_code().to_string());
                let operator_name = self.user_repo.find_by_id(o.operator_id()).await.map_err(ServiceError::from)?.map(|u| u.username().to_string());

                let order_status = o.order_status().to_string();
                let complete_time = o.complete_time().clone();
                let order_info = OutOrderListItem {
                    order_id: o.order_id().to_string(),
                    warehouse_id: o.warehouse_id().to_string(),
                    warehouse_name: warehouse_name.unwrap_or_default(),
                    customer_id: o.customer_id(),
                    customer_name: customer_name.unwrap_or_default(),
                    operator_id: o.operator_id().to_string(),
                    operator: operator_name.unwrap_or_default(),
                    contact: o.contact().as_ref().map(|c| c.as_str().to_string()),
                    phone: o.phone().as_ref().map(|p| p.as_str().to_string()),
                    order_status,
                    order_type: Some(o.order_type().as_str().to_string()),
                    out_time: o.out_time().clone(),
                    complete_time,
                    remark: o.remark().clone(),
                };

                Ok(Some(OutOrderDetailItem { order: order_info, items: enriched_items }))
            }
            None => Ok(None),
        }
    }

    /// 创建出库单（事务：创建订单 + 创建明细）
    pub async fn create_out_order(&self, operator_id: &str, payload: &CreateOutOrderRequest) -> Result<(), ServiceError> {
        let existing = self.out_order_repo.find_by_id(&payload.order_id).await.map_err(ServiceError::from)?;
        if existing.is_some() {
            return Err(ServiceError::AlreadyExists(format!("出库单号【{}】已存在", payload.order_id)));
        }

        let contact_vo = payload.contact.clone().map(|s| Contact::new(s).unwrap());
        let phone_vo = payload.phone.clone().map(|s| Phone::new(s).unwrap());
        let mut order = OutOrder::new(
            payload.order_id.clone(),
            payload.warehouse_id.clone(),
            payload.customer_id,
            operator_id.to_string(),
            contact_vo,
            phone_vo,
            Some(payload.out_time),
            payload.remark.clone(),
        );

        for item in &payload.items {
            let order_item = OutOrderItem::new(
                item.material_id.clone(),
                item.num,
                item.purchase_price,
                item.sell_price,
            )?;
            order.add_item(order_item).map_err(|e| ServiceError::Validation(e.to_string()))?;
        }

        OutOrderDomainService::validate_out_order_items(order.get_items()).map_err(ServiceError::from)?;

        let txn = self.transaction_factory.begin_transaction().await.map_err(|e| ServiceError::Database(e))?;

        self.out_order_repo.create_in_txn(&txn, order.clone()).await.map_err(ServiceError::from)?;

        for item in order.get_items() {
            self.out_order_item_repo.create_in_txn(&txn, item.clone()).await.map_err(ServiceError::from)?;
        }

        txn.commit().await.map_err(|e| ServiceError::Database(e))?;
        Ok(())
    }

    /// 审核出库单（使用 OutOrderDomainService 编排跨实体业务规则）
    pub async fn audit_out_order(&self, payload: &AuditOrderRequest) -> Result<(), ServiceError> {
        let mut order = self.out_order_repo.find_by_id(&payload.order_id).await.map_err(ServiceError::from)?
            .ok_or(ServiceError::NotFound("出库单不存在".to_string()))?;

        if OrderStatus::from_str(&payload.order_status) == Some(OrderStatus::Approved) {
            let txn = self.transaction_factory.begin_transaction().await.map_err(|e| ServiceError::Database(e))?;

            let items = self.out_order_item_repo.find_by_order_id_in_txn(&txn, &payload.order_id).await.map_err(ServiceError::from)?;

            let mut existing_stocks = Vec::with_capacity(items.len());
            for item in &items {
                let stock = self.stock_repo.find_by_material_and_warehouse_in_txn(&txn, item.material_id(), order.warehouse_id()).await.map_err(ServiceError::from)?;
                existing_stocks.push(stock);
            }

            let changes = OutOrderDomainService::process_outbound_approval(&mut order, &items, existing_stocks)
                .map_err(ServiceError::from)?;

            for change in changes {
                self.stock_repo.update_in_txn(&txn, change.stock).await.map_err(ServiceError::from)?;
            }

            self.out_order_repo.update_in_txn(&txn, order).await.map_err(ServiceError::from)?;
            txn.commit().await.map_err(|e| ServiceError::Database(e))?;
        } else {
            order.reject().map_err(|e| ServiceError::Validation(e.to_string()))?;
            self.out_order_repo.update(order).await.map_err(ServiceError::from)?;
        }

        Ok(())
    }

    /// 删除出库单及其明细（事务：先删明细再删订单）
    pub async fn delete_out_order_with_items(&self, order_id: &str) -> Result<(), ServiceError> {
        let order = self.out_order_repo.find_by_id(order_id).await.map_err(ServiceError::from)?;
        match order {
            Some(_) => {
                let txn = self.transaction_factory.begin_transaction().await.map_err(|e| ServiceError::Database(e))?;
                self.out_order_item_repo.delete_by_order_id_in_txn(&txn, order_id).await.map_err(ServiceError::from)?;
                self.out_order_repo.delete_by_id_in_txn(&txn, order_id).await.map_err(ServiceError::from)?;
                txn.commit().await.map_err(|e| ServiceError::Database(e))?;
                Ok(())
            }
            None => Err(ServiceError::NotFound("出库单不存在".to_string())),
        }
    }

    /// 查询出库单的所有明细
    pub async fn list_out_order_items(&self, order_id: &str) -> Result<Vec<OutOrderItem>, ServiceError> {
        self.out_order_item_repo.find_by_order_id(order_id).await.map_err(ServiceError::from)
    }

    // --- 批量名称查询辅助方法 ---
    async fn get_customer_names_by_ids(&self, ids: &[i32]) -> Result<std::collections::HashMap<i32, String>, ServiceError> {
        let list = self.customer_repo.find_by_ids(ids).await.map_err(ServiceError::from)?;
        Ok(list.into_iter().map(|c| (c.customer_id(), c.customer_name().to_string())).collect())
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
