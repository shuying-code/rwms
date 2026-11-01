// ============================================
// 路由配置模块
// 定义所有 API 端点的路径、HTTP 方法与处理函数的映射关系
// ============================================

use axum::{
    middleware, // Axum 中间件模块，用于添加认证等中间件
    routing::{get, post, delete}, // HTTP 方法路由构造器：get=查询, post=创建/更新, delete=删除
    Router, // Axum 路由器，用于组合多个路由并添加中间件
};
use tower_http::services::ServeDir; // 静态文件服务，用于托管前端 HTML/CSS/JS 文件

use crate::{
    interfaces::handlers::{
        auth_controller as auth, // 认证控制器，别名 auth，处理登录/登出
        material_controller as material, // 物资控制器，别名 material，处理物资 CRUD
        category_controller as category, // 分类控制器，别名 category，处理分类 CRUD
        warehouse_controller as warehouse, // 仓库控制器，别名 warehouse，处理仓库 CRUD
        supplier_controller as supplier, // 供应商控制器，别名 supplier，处理供应商 CRUD
        customer_controller as customer, // 客户控制器，别名 customer，处理客户 CRUD
        in_order_controller as in_order, // 入库单控制器，别名 in_order，处理入库单操作
        out_order_controller as out_order, // 出库单控制器，别名 out_order，处理出库单操作
        stock_controller as stock, // 库存控制器，别名 stock，处理库存查询
    },
    infrastructure::config::auth_middleware, // JWT 认证中间件，用于校验受保护路由的请求
    state::AppState, // 应用共享状态，包含所有应用层服务实例
};

/// 创建公开路由（无需 JWT 认证）
pub fn create_public_routes(state: AppState) -> Router {
    Router::new() // 创建新的路由器实例
        .route("/", get(home)) // 首页路由，GET /，返回系统启动成功信息
        .route("/health", get(health_check)) // 健康检查路由，GET /health，用于 K8s/Docker 健康探测
        .route("/api/login", post(auth::login)) // 登录路由，POST /api/login，提交用户名密码获取 JWT 令牌
        .with_state(state) // 将应用状态注入路由器，使处理器可通过 State 提取器获取
}

/// 首页处理函数，返回系统启动成功信息
async fn home() -> axum::Json<serde_json::Value> {
    use serde_json::json; // 导入 JSON 构建宏
    axum::Json(json!({ // 返回 JSON 格式的系统信息
        "code": 200, // 状态码 200 表示成功
        "message": "WMS 仓库管理系统启动成功", // 系统启动成功提示
        "data": null // 无额外数据
    }))
}

/// 健康检查处理函数，返回服务运行状态
async fn health_check() -> axum::Json<serde_json::Value> {
    use serde_json::json; // 导入 JSON 构建宏
    axum::Json(json!({ // 返回 JSON 格式的健康状态
        "status": "UP", // 服务状态为运行中
        "service": "WMS" // 服务名称标识
    }))
}

/// 创建受保护的路由（需要 JWT 认证）
pub fn create_protected_routes(state: AppState) -> Router {
    Router::new()
        // ===== 认证相关路由 =====
        .route("/api/logout", post(auth::logout)) // 登出路由，POST /api/logout，使 JWT 令牌失效

        // ===== 物资管理路由 =====
        .route("/api/materials", get(material::list_materials)) // 获取物资列表，GET /api/materials，支持分页查询
        .route("/api/materials", post(material::create_material)) // 创建物资，POST /api/materials，提交物资信息
        .route("/api/materials/:id", get(material::get_material)) // 获取物资详情，GET /api/materials/:id，按物资编号查询
        .route("/api/materials/update", post(material::update_material)) // 更新物资，POST /api/materials/update，提交更新信息
        .route("/api/materials/:id", delete(material::delete_material)) // 删除物资，DELETE /api/materials/:id，按物资编号删除

        // ===== 分类管理路由 =====
        .route("/api/categories", get(category::list_categories)) // 获取分类列表，GET /api/categories，支持分页查询
        .route("/api/categories", post(category::create_category)) // 创建分类，POST /api/categories，提交分类信息
        .route("/api/categories/update", post(category::update_category)) // 更新分类，POST /api/categories/update，提交更新信息
        .route("/api/categories/:id", get(category::get_category)) // 获取分类详情，GET /api/categories/:id，按分类 ID 查询
        .route("/api/categories/:id", delete(category::delete_category)) // 删除分类，DELETE /api/categories/:id，按分类 ID 删除

        // ===== 仓库管理路由 =====
        .route("/api/warehouses", get(warehouse::list_warehouses)) // 获取仓库列表，GET /api/warehouses，支持分页查询
        .route("/api/warehouses", post(warehouse::create_warehouse)) // 创建仓库，POST /api/warehouses，提交仓库信息
        .route("/api/warehouses/update", post(warehouse::update_warehouse)) // 更新仓库，POST /api/warehouses/update，提交更新信息
        .route("/api/warehouses/:id", get(warehouse::get_warehouse)) // 获取仓库详情，GET /api/warehouses/:id，按仓库编号查询
        .route("/api/warehouses/:id", delete(warehouse::delete_warehouse)) // 删除仓库，DELETE /api/warehouses/:id，按仓库编号删除

        // ===== 供应商管理路由 =====
        .route("/api/suppliers", get(supplier::list_suppliers)) // 获取供应商列表，GET /api/suppliers，支持分页查询
        .route("/api/suppliers", post(supplier::create_supplier)) // 创建供应商，POST /api/suppliers，提交供应商信息
        .route("/api/suppliers/update", post(supplier::update_supplier)) // 更新供应商，POST /api/suppliers/update，提交更新信息
        .route("/api/suppliers/:id", get(supplier::get_supplier)) // 获取供应商详情，GET /api/suppliers/:id，按供应商 ID 查询
        .route("/api/suppliers/:id", delete(supplier::delete_supplier)) // 删除供应商，DELETE /api/suppliers/:id，按供应商 ID 删除

        // ===== 客户管理路由 =====
        .route("/api/customers", get(customer::list_customers)) // 获取客户列表，GET /api/customers，支持分页查询
        .route("/api/customers", post(customer::create_customer)) // 创建客户，POST /api/customers，提交客户信息
        .route("/api/customers/update", post(customer::update_customer)) // 更新客户，POST /api/customers/update，提交更新信息
        .route("/api/customers/:id", get(customer::get_customer)) // 获取客户详情，GET /api/customers/:id，按客户 ID 查询
        .route("/api/customers/:id", delete(customer::delete_customer)) // 删除客户，DELETE /api/customers/:id，按客户 ID 删除

        // ===== 入库单管理路由 =====
        .route("/api/in-orders", get(in_order::list_in_orders)) // 获取入库单列表，GET /api/in-orders，支持分页查询
        .route("/api/in-orders", post(in_order::create_in_order)) // 创建入库单，POST /api/in-orders，提交入库单及明细
        .route("/api/in-orders/:id", get(in_order::get_in_order)) // 获取入库单详情，GET /api/in-orders/:id，含明细列表
        .route("/api/in-orders/audit", post(in_order::audit_in_order)) // 审核入库单，POST /api/in-orders/audit，通过/拒绝
        .route("/api/in-orders/:id", delete(in_order::delete_in_order)) // 删除入库单，DELETE /api/in-orders/:id，级联删除明细
        .route("/api/in-orders/:id/items", get(in_order::list_in_order_items)) // 获取入库单明细，GET /api/in-orders/:id/items

        // ===== 出库单管理路由 =====
        .route("/api/out-orders", get(out_order::list_out_orders)) // 获取出库单列表，GET /api/out-orders，支持分页查询
        .route("/api/out-orders", post(out_order::create_out_order)) // 创建出库单，POST /api/out-orders，提交出库单及明细
        .route("/api/out-orders/:id", get(out_order::get_out_order)) // 获取出库单详情，GET /api/out-orders/:id，含明细列表
        .route("/api/out-orders/audit", post(out_order::audit_out_order)) // 审核出库单，POST /api/out-orders/audit，通过/拒绝
        .route("/api/out-orders/:id", delete(out_order::delete_out_order)) // 删除出库单，DELETE /api/out-orders/:id，级联删除明细
        .route("/api/out-orders/:id/items", get(out_order::list_out_order_items)) // 获取出库单明细，GET /api/out-orders/:id/items

        // ===== 库存查询路由 =====
        .route("/api/stocks", get(stock::list_stocks)) // 获取库存列表，GET /api/stocks，支持多条件过滤和分页

        .with_state(state.clone()) // 将应用状态克隆注入路由器（需克隆因 state 还要传给中间件）
        .layer(middleware::from_fn_with_state(state, auth_middleware::auth_middleware)) // 添加 JWT 认证中间件，校验所有受保护路由的请求
}

/// 创建完整路由
pub fn create_router(state: AppState) -> Router {
    let public_routes = create_public_routes(state.clone()); // 创建公开路由组（登录、健康检查等，无需认证）
    let protected_routes = create_protected_routes(state.clone()); // 创建受保护路由组（业务操作，需 JWT 认证）
    public_routes
        .merge(protected_routes) // 合并公开路由和受保护路由
        .fallback_service(ServeDir::new("static")) // 设置静态文件回退服务，未匹配的路径从 static 目录查找前端文件
}
