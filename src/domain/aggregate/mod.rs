// ============================================
// 聚合模块（Aggregate）
// 包含聚合根、实体、值对象
// 富领域模型，包含业务行为和状态转换
// 与 ORM 模型解耦，纯业务逻辑
// ============================================

// --- 值对象（Value Object） ---
pub mod order_status;        // 值对象：订单状态（Pending/Approved/Rejected）
pub mod order_type;          // 值对象：订单类型（Inbound/Outbound）
pub mod account_status;      // 值对象：账号状态（Active/Disabled）
pub mod user_role;           // 值对象：用户角色（SuperAdmin/WarehouseAdmin/Operator）
pub mod phone;               // 值对象：电话号码（含格式验证）
pub mod contact;             // 值对象：联系人姓名（含非空验证）
pub mod unit;                // 值对象：计量单位（含非空验证）
pub mod warehouse_code;      // 值对象：仓库编码（含非空验证）

// --- 聚合根（Aggregate Root） ---
pub mod sys_user;            // 聚合根：系统用户（含密码验证、状态管理）
pub mod warehouse;           // 聚合根：仓库（含容量分配/释放/调整）
pub mod material_category;   // 聚合根：物资分类（CRUD 实体）
pub mod material;            // 聚合根：物资（含价格验证）
pub mod supplier;            // 聚合根：供应商（CRUD 实体）
pub mod customer;            // 聚合根：客户（CRUD 实体）
pub mod in_order;            // 聚合根：入库单（含状态转换、明细管理）
pub mod out_order;           // 聚合根：出库单（含状态转换、明细管理）
pub mod stock;               // 聚合根：库存（含入库增加/出库扣减）

// --- 实体（Entity） ---
pub mod in_order_item;       // 实体：入库单明细，属于 InOrder 聚合
pub mod out_order_item;      // 实体：出库单明细，属于 OutOrder 聚合
