// ============================================
// 应用状态模块（组合根 / Composition Root）
// DDD 架构：此处是唯一创建 Infrastructure 具体类型的地方
// 所有依赖在此组装后注入 Application 层服务
// ============================================

use std::sync::Arc;
use sea_orm::DatabaseConnection;

// Infrastructure 具体实现
use crate::infrastructure::repository::{
    SeaOrmTransactionFactory,
    user_repo_impl::SeaOrmUserRepository,
    category_repo_impl::SeaOrmCategoryRepository,
    customer_repo_impl::SeaOrmCustomerRepository,
    supplier_repo_impl::SeaOrmSupplierRepository,
    warehouse_repo_impl::SeaOrmWarehouseRepository,
    material_repo_impl::SeaOrmMaterialRepository,
    stock_repo_impl::SeaOrmStockRepository,
    in_order_repo_impl::SeaOrmInOrderRepository,
    in_order_item_repo_impl::SeaOrmInOrderItemRepository,
    out_order_repo_impl::SeaOrmOutOrderRepository,
    out_order_item_repo_impl::SeaOrmOutOrderItemRepository,
};

// Application 服务
use crate::application::{
    auth_service::AuthApplicationService,
    category_service::CategoryApplicationService,
    customer_service::CustomerApplicationService,
    supplier_service::SupplierApplicationService,
    warehouse_service::WarehouseApplicationService,
    material_service::MaterialApplicationService,
    stock_service::StockApplicationService,
    in_order_service::InOrderApplicationService,
    out_order_service::OutOrderApplicationService,
};

/// 应用共享状态
/// 持有所有 Application 层服务实例，Handler 通过 Axum State 提取器获取
#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthApplicationService,
    pub category_service: CategoryApplicationService,
    pub customer_service: CustomerApplicationService,
    pub supplier_service: SupplierApplicationService,
    pub warehouse_service: WarehouseApplicationService,
    pub material_service: MaterialApplicationService,
    pub stock_service: StockApplicationService,
    pub in_order_service: InOrderApplicationService,
    pub out_order_service: OutOrderApplicationService,
}

impl AppState {
    /// 组合根：创建所有 Infrastructure 实例并注入到 Application 服务
    pub fn new(db: DatabaseConnection, jwt_secret: String) -> Self {
        // --- 创建事务工厂 ---
        let transaction_factory = Arc::new(SeaOrmTransactionFactory::new(db.clone()));

        // --- 创建仓储实现 ---
        let user_repo           = Arc::new(SeaOrmUserRepository::new(db.clone()));
        let category_repo       = Arc::new(SeaOrmCategoryRepository::new(db.clone()));
        let customer_repo       = Arc::new(SeaOrmCustomerRepository::new(db.clone()));
        let supplier_repo       = Arc::new(SeaOrmSupplierRepository::new(db.clone()));
        let warehouse_repo      = Arc::new(SeaOrmWarehouseRepository::new(db.clone()));
        let material_repo       = Arc::new(SeaOrmMaterialRepository::new(db.clone()));
        let stock_repo          = Arc::new(SeaOrmStockRepository::new(db.clone()));
        let in_order_repo       = Arc::new(SeaOrmInOrderRepository::new(db.clone()));
        let in_order_item_repo  = Arc::new(SeaOrmInOrderItemRepository::new(db.clone()));
        let out_order_repo      = Arc::new(SeaOrmOutOrderRepository::new(db.clone()));
        let out_order_item_repo = Arc::new(SeaOrmOutOrderItemRepository::new(db.clone()));

        // --- 组装 Application 服务，注入依赖 ---
        Self {
            auth_service: AuthApplicationService::new(
                user_repo.clone(),
                jwt_secret,
            ),
            category_service: CategoryApplicationService::new(
                category_repo.clone(),
            ),
            customer_service: CustomerApplicationService::new(
                customer_repo.clone(),
            ),
            supplier_service: SupplierApplicationService::new(
                supplier_repo.clone(),
            ),
            warehouse_service: WarehouseApplicationService::new(
                warehouse_repo.clone(),
            ),
            material_service: MaterialApplicationService::new(
                material_repo.clone(),
                category_repo.clone(),
            ),
            stock_service: StockApplicationService::new(
                stock_repo.clone(),
                material_repo.clone(),
                warehouse_repo.clone(),
            ),
            in_order_service: InOrderApplicationService::new(
                transaction_factory.clone(),
                in_order_repo,
                in_order_item_repo,
                stock_repo.clone(),
                supplier_repo.clone(),
                warehouse_repo.clone(),
                user_repo.clone(),
                material_repo.clone(),
            ),
            out_order_service: OutOrderApplicationService::new(
                transaction_factory,
                out_order_repo,
                out_order_item_repo,
                stock_repo,
                customer_repo.clone(),
                warehouse_repo,
                user_repo,
                material_repo,
            ),
        }
    }
}
