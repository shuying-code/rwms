// ============================================
// Application 应用层
// 业务编排、事务管理、领域实体-DTO 转换
// ============================================

pub mod error;
pub mod auth_service;           // 认证应用服务（登录/登出/JWT）
pub mod category_service;      // 物资分类应用服务（CRUD）
pub mod customer_service;      // 客户应用服务（CRUD）
pub mod supplier_service;      // 供应商应用服务（CRUD）
pub mod warehouse_service;     // 仓库应用服务（CRUD）
pub mod material_service;      // 物资应用服务（CRUD + 分类名称关联）
pub mod stock_service;         // 库存应用服务（查询 + 名称关联）
pub mod in_order_service;      // 入库单应用服务（事务编排 + 审核流程）
pub mod out_order_service;     // 出库单应用服务（事务编排 + 审核流程）
