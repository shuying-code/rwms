// ============================================
// 领域服务模块（Domain Service）
// 封装跨聚合的业务逻辑
// ============================================

pub mod stock_domain_service;          // 库存领域服务：入库增加/出库扣减库存
pub mod in_order_domain_service;       // 入库单领域服务：入库校验 + 审核流程编排
pub mod out_order_domain_service;      // 出库单领域服务：出库校验 + 库存预检 + 审核流程编排
pub mod warehouse_domain_service;      // 仓库领域服务：仓库容量管理
