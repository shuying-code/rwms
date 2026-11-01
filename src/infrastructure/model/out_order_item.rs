// ============================================
// 出库单明细实体
// 对应数据库表: out_order_item
// 描述: 记录出库单中每种物资的详细信息
// ============================================

// 导入 SeaORM 框架核心类型，包含 EntityTrait、DeriveEntityModel、ActiveModelBehavior 等
use sea_orm::entity::prelude::*;
// 导入 serde 序列化/反序列化 trait，用于 JSON 交互
use serde::{Deserialize, Serialize};
// 导入 rust_decimal::Decimal 高精度十进制类型，用于精确表示金额，避免浮点数精度丢失
use rust_decimal::Decimal;

/// 出库单明细实体模型
///
/// # 字段说明
/// * `item_id` - 明细ID（主键，自增）
/// * `order_id` - 出库单号（关联出库单表）
/// * `material_id` - 物资编号（关联物资表）
/// * `num` - 出库数量
/// * `purchase_price` - 采购单价
/// * `sell_price` - 销售单价
/// * `total_amount` - 总金额（销售单价 × 数量）
// Clone - 支持克隆，便于在多处使用同一明细数据
// Debug - 支持调试输出，方便日志打印
// PartialEq - 支持相等比较，用于测试断言
// DeriveEntityModel - SeaORM 宏，自动生成 Entity、Column、PrimaryKey 等关联类型
// Serialize - 支持 serde 序列化，可将结构体转为 JSON 等格式
// Deserialize - 支持 serde 反序列化，可将 JSON 等格式解析为结构体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
// 指定该实体映射到数据库中的 "out_order_item" 表
#[sea_orm(table_name = "out_order_item")]
pub struct Model {
    /// 明细ID（主键，自增）
    // sea_orm(primary_key) 标记此字段为主键，默认 auto_increment = true（自增）
    #[sea_orm(primary_key)]
    // i32 类型对应数据库中的 INT，作为明细唯一标识，由数据库自动生成
    pub item_id: i32,
    /// 出库单号（关联出库单表）
    // String 类型，存储关联的出库单号，逻辑外键关联 out_order 表的 order_id
    // 一个出库单可包含多条明细记录
    pub order_id: String,
    /// 物资编号（关联物资表）
    // String 类型，存储物资编号，逻辑外键关联 material 表的 material_id
    pub material_id: String,
    /// 出库数量
    // i32 类型对应数据库中的 INT，存储本次出库的物资数量
    pub num: i32,
    /// 采购单价
    // Decimal 类型对应数据库中的 DECIMAL，存储采购单价
    // 使用 Decimal 而非 f64 是为了精确表示金额，避免浮点数计算精度丢失
    // 记录出库时的采购成本价，便于计算利润
    pub purchase_price: Decimal,
    /// 销售单价
    // Decimal 类型对应数据库中的 DECIMAL，存储销售单价
    // 用于计算出库总金额
    pub sell_price: Decimal,
    /// 总金额（销售单价 × 数量）
    // Option<Decimal> 对应数据库中的 DECIMAL NULLABLE，总金额可为空
    // 计算公式：total_amount = sell_price × num（注意：出库单用销售单价计算，与入库单用采购单价不同）
    // 为 Option 类型是为了兼容历史数据或尚未计算的记录
    pub total_amount: Option<Decimal>,
}

/// 实体关系定义（当前无关联关系）
// Copy - 支持按位复制（枚举体型）
// Clone - 支持克隆
// Debug - 支持调试输出
// EnumIter - SeaORM 宏，生成枚举迭代器
// DeriveRelation - SeaORM 宏，自动生成 RelationTrait 实现
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// 空枚举表示该实体当前没有定义与其他实体的关联关系
// 虽然逻辑上关联了 out_order、material 等表，但 SeaORM 层面未声明外键关系
pub enum Relation {}

/// ActiveModel 行为实现
// ActiveModelBehavior 是 SeaORM 的 trait，为空实现（默认行为）
// 提供 insert()、update()、delete() 等数据库操作方法的基础支持
impl ActiveModelBehavior for ActiveModel {}
