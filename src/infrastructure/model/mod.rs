// ============================================
// 持久化模型模块（Persistence Model / PO）
// SeaORM DeriveEntityModel 宏生成的数据库表映射
// 与领域聚合解耦，仅 Infrastructure 层使用
// ============================================

// 系统用户数据模型子模块
pub mod sys_user;
// 仓库数据模型子模块
pub mod warehouse;
// 物资分类数据模型子模块
pub mod material_category;
// 物资数据模型子模块
pub mod material;
// 供应商数据模型子模块
pub mod supplier;
// 客户数据模型子模块
pub mod customer;
// 入库单数据模型子模块
pub mod in_order;
// 入库单明细数据模型子模块
pub mod in_order_item;
// 出库单数据模型子模块
pub mod out_order;
// 出库单明细数据模型子模块
pub mod out_order_item;
// 库存数据模型子模块
pub mod stock;
