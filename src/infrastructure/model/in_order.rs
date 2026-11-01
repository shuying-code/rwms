// ============================================
// 入库单实体
// 对应数据库表: in_order
// 描述: 记录物资入库的订单信息
// ============================================

// 导入 SeaORM 框架核心类型，包含 EntityTrait、DeriveEntityModel、ActiveModelBehavior、DateTimeUtc 等
use sea_orm::entity::prelude::*;
// 导入 serde 序列化/反序列化 trait，用于 JSON 交互
use serde::{Deserialize, Serialize};

/// 入库单实体模型
///
/// # 字段说明
/// * `order_id` - 入库单号（主键，业务生成）
/// * `warehouse_id` - 仓库编号（关联仓库表）
/// * `supplier_id` - 供应商ID（关联供应商表）
/// * `operator_id` - 操作人ID（关联用户表）
/// * `contact` - 联系人
/// * `phone` - 联系电话
/// * `order_status` - 订单状态（待审核/审核通过/审核不通过）
/// * `order_type` - 订单类型（入库）
/// * `purchase_time` - 采购时间
/// * `complete_time` - 完成时间（审核时间）
/// * `remark` - 备注
// Clone - 支持克隆，便于在多处使用同一入库单数据
// Debug - 支持调试输出，方便日志打印
// PartialEq - 支持相等比较，用于测试断言
// DeriveEntityModel - SeaORM 宏，自动生成 Entity、Column、PrimaryKey 等关联类型
// Serialize - 支持 serde 序列化，可将结构体转为 JSON 等格式
// Deserialize - 支持 serde 反序列化，可将 JSON 等格式解析为结构体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
// 指定该实体映射到数据库中的 "in_order" 表
#[sea_orm(table_name = "in_order")]
pub struct Model {
    /// 入库单号（主键，业务生成，如：IN202401010001）
    // primary_key - 标记此字段为主键
    // auto_increment = false - 主键不由数据库自增，而是由业务层生成（如按规则拼接单号）
    #[sea_orm(primary_key, auto_increment = false)]
    // String 类型对应数据库中的 VARCHAR，存储入库单号（业务主键）
    pub order_id: String,
    /// 仓库编号（关联仓库表）
    // String 类型，存储目标仓库的编号，逻辑外键关联 warehouse 表的 warehouse_id
    pub warehouse_id: String,
    /// 供应商ID（关联供应商表）
    // i32 类型，存储供应商ID，逻辑外键关联 supplier 表的 supplier_id
    pub supplier_id: i32,
    /// 操作人ID（关联用户表）
    // String 类型，存储操作人ID，逻辑外键关联 sys_user 表的 user_id
    pub operator_id: String,
    /// 联系人
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，联系人可为空
    pub contact: Option<String>,
    /// 联系电话
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，联系电话可为空
    pub phone: Option<String>,
    /// 订单状态（待审核/审核通过/审核不通过）
    // String 类型，存储订单当前审核状态，必填字段
    // 典型值："待审核"、"审核通过"、"审核不通过"
    pub order_status: String,
    /// 订单类型（入库）
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，订单类型可为空
    // 典型值："入库"，预留字段便于扩展其他入库类型
    pub order_type: Option<String>,
    /// 采购时间
    // Option<DateTimeUtc> 对应数据库中的 TIMESTAMP NULLABLE，存储采购发生的时间
    // DateTimeUtc 是 SeaORM 提供的 UTC 时区日期时间类型
    pub purchase_time: Option<DateTimeUtc>,
    /// 完成时间（审核时间）
    // Option<DateTimeUtc> 对应数据库中的 TIMESTAMP NULLABLE，存储审核完成的时间
    // 入库单审核通过或拒绝时记录该时间
    pub complete_time: Option<DateTimeUtc>,
    /// 备注
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，备注信息可为空
    pub remark: Option<String>,
}

/// 实体关系定义（当前无关联关系）
// Copy - 支持按位复制（枚举体型）
// Clone - 支持克隆
// Debug - 支持调试输出
// EnumIter - SeaORM 宏，生成枚举迭代器
// DeriveRelation - SeaORM 宏，自动生成 RelationTrait 实现
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// 空枚举表示该实体当前没有定义与其他实体的关联关系
// 虽然逻辑上关联了 warehouse、supplier、sys_user 等表，但 SeaORM 层面未声明外键关系
pub enum Relation {}

/// ActiveModel 行为实现
// ActiveModelBehavior 是 SeaORM 的 trait，为空实现（默认行为）
// 提供 insert()、update()、delete() 等数据库操作方法的基础支持
impl ActiveModelBehavior for ActiveModel {}
