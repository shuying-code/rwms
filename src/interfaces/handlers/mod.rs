// ============================================
// HTTP 处理器模块
// ============================================

pub mod auth_controller; // 认证处理器，处理用户登录/登出请求
pub mod category_controller; // 分类处理器，处理物资分类 CRUD 请求
pub mod customer_controller; // 客户处理器，处理客户 CRUD 请求
pub mod supplier_controller; // 供应商处理器，处理供应商 CRUD 请求
pub mod warehouse_controller; // 仓库处理器，处理仓库 CRUD 请求
pub mod material_controller; // 物资处理器，处理物资 CRUD 请求
pub mod in_order_controller; // 入库单处理器，处理入库单创建/审核/删除请求
pub mod out_order_controller; // 出库单处理器，处理出库单创建/审核/删除请求
pub mod stock_controller; // 库存处理器，处理库存查询请求
