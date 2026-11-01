// ============================================
// 物资实体模块
// 定义物资表结构和 ORM 映射
// 对应数据库表: material
// ============================================

// 导入 SeaORM 框架核心类型，包含 EntityTrait、DeriveEntityModel、ActiveModelBehavior、DateTimeUtc 等
use sea_orm::entity::prelude::*;
// 导入 serde 序列化/反序列化 trait，用于 JSON 交互
use serde::{Deserialize, Serialize};
// 导入 rust_decimal::Decimal 高精度十进制类型，用于精确表示金额，避免浮点数精度丢失
use rust_decimal::Decimal;

/// 物资实体模型
///
/// # 字段说明
/// * `material_id` - 物资编号（主键，业务唯一标识）
/// * `material_name` - 物资名称
/// * `category_id` - 分类ID（外键，关联 material_category 表）
/// * `unit` - 计量单位（如：个、件、箱等）
/// * `purchase_price` - 采购单价
/// * `sell_price` - 销售单价
/// * `total_stock` - 总库存数量（汇总所有仓库）
/// * `create_time` - 创建时间
/// * `update_time` - 更新时间
// Clone - 支持克隆，便于在多处使用同一物资数据
// Debug - 支持调试输出，方便日志打印
// PartialEq - 支持相等比较，用于测试断言
// DeriveEntityModel - SeaORM 宏，自动生成 Entity、Column、PrimaryKey 等关联类型
// Serialize - 支持 serde 序列化，可将结构体转为 JSON 等格式
// Deserialize - 支持 serde 反序列化，可将 JSON 等格式解析为结构体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
// 指定该实体映射到数据库中的 "material" 表
#[sea_orm(table_name = "material")]
pub struct Model {
    /// 物资编号（主键，业务唯一标识）
    // primary_key - 标记此字段为主键
    // auto_increment = false - 主键不由数据库自增，而是由业务层生成（如按规则编码）
    #[sea_orm(primary_key, auto_increment = false)]
    // String 类型对应数据库中的 VARCHAR，存储物资编号（业务主键，如 "M001"）
    pub material_id: String,
    /// 物资名称
    // String 类型对应数据库中的 VARCHAR，存储物资名称，为必填字段
    pub material_name: String,
    /// 分类ID（外键）
    // i32 类型，存储物资所属分类的ID，逻辑外键关联 material_category 表的 category_id
    pub category_id: i32,
    /// 计量单位
    // String 类型对应数据库中的 VARCHAR，存储计量单位（如："个"、"件"、"箱"、"台"等）
    pub unit: String,
    /// 采购单价
    // Decimal 类型对应数据库中的 DECIMAL，存储采购单价
    // 使用 Decimal 而非 f64 是为了精确表示金额，避免浮点数计算精度丢失
    pub purchase_price: Decimal,
    /// 销售单价
    // Decimal 类型对应数据库中的 DECIMAL，存储销售单价
    // 用于出库时计算销售总金额
    pub sell_price: Decimal,
    /// 总库存数量
    // Option<i32> 对应数据库中的 INT NULLABLE，总库存数量可为空
    // 该字段为汇总值，由所有仓库中该物资的库存数量累加得到
    // 为 Option 类型是为了兼容尚未计算或初始化的记录
    pub total_stock: Option<i32>,
    /// 创建时间
    // Option<DateTimeUtc> 对应数据库中的 TIMESTAMP NULLABLE，存储物资记录的创建时间
    // DateTimeUtc 是 SeaORM 提供的 UTC 时区日期时间类型
    pub create_time: Option<DateTimeUtc>,
    /// 更新时间
    // Option<DateTimeUtc> 对应数据库中的 TIMESTAMP NULLABLE，存储物资记录的最后更新时间
    // 当物资信息（价格、库存等）变更时更新此字段
    pub update_time: Option<DateTimeUtc>,
}

/// 实体关系定义（当前无关联关系）
// Copy - 支持按位复制（枚举体型）
// Clone - 支持克隆
// Debug - 支持调试输出
// EnumIter - SeaORM 宏，生成枚举迭代器
// DeriveRelation - SeaORM 宏，自动生成 RelationTrait 实现
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// 空枚举表示该实体当前没有定义与其他实体的关联关系
// 虽然逻辑上关联了 material_category 等表，但 SeaORM 层面未声明外键关系
pub enum Relation {}

/// ActiveModel 行为实现
// ActiveModelBehavior 是 SeaORM 的 trait，为空实现（默认行为）
// 提供 insert()、update()、delete() 等数据库操作方法的基础支持
impl ActiveModelBehavior for ActiveModel {}
