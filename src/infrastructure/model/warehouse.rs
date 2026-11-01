// ============================================
// 仓库实体模块
// 对应数据库表: warehouse
// ============================================

// 导入 SeaORM 框架核心类型，包含 EntityTrait、DeriveEntityModel、ActiveModelBehavior、DateTimeUtc 等
use sea_orm::entity::prelude::*;
// 导入 serde 序列化/反序列化 trait，用于 JSON 交互
use serde::{Deserialize, Serialize};

/// 仓库实体模型
///
/// # 字段说明
/// * `warehouse_id` - 仓库ID（主键）
/// * `warehouse_code` - 仓库编号
/// * `manager` - 仓库管理员
/// * `max_space` - 最大容量
/// * `remain_space` - 剩余容量
/// * `create_time` - 创建时间
// Clone - 支持克隆，便于在多处使用同一仓库数据
// Debug - 支持调试输出，方便日志打印
// PartialEq - 支持相等比较，用于测试断言
// DeriveEntityModel - SeaORM 宏，自动生成 Entity、Column、PrimaryKey 等关联类型
// Serialize - 支持 serde 序列化，可将结构体转为 JSON 等格式
// Deserialize - 支持 serde 反序列化，可将 JSON 等格式解析为结构体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
// 指定该实体映射到数据库中的 "warehouse" 表
#[sea_orm(table_name = "warehouse")]
pub struct Model {
    /// 仓库ID（主键）
    // primary_key - 标记此字段为主键
    // auto_increment = false - 主键不由数据库自增，而是由业务层生成
    #[sea_orm(primary_key, auto_increment = false)]
    // String 类型对应数据库中的 VARCHAR，存储仓库唯一标识
    pub warehouse_id: String,
    /// 仓库编号
    // String 类型对应数据库中的 VARCHAR，存储仓库编号（如 "WH001"）
    // 与 warehouse_id 可能相同或不同，具体取决于业务设计
    pub warehouse_code: String,
    /// 仓库管理员
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，仓库管理员姓名可为空
    pub manager: Option<String>,
    /// 最大容量
    // i32 类型对应数据库中的 INT，存储仓库最大存储容量（如可存放的物资总数量）
    // 用于库存预警：当剩余容量不足时提醒用户
    pub max_space: i32,
    /// 剩余容量
    // i32 类型对应数据库中的 INT，存储仓库当前剩余可用容量
    // 入库时减少，出库时增加，应满足：remain_space <= max_space 且 remain_space >= 0
    pub remain_space: i32,
    /// 创建时间
    // Option<DateTimeUtc> 对应数据库中的 TIMESTAMP NULLABLE，存储仓库记录的创建时间
    // DateTimeUtc 是 SeaORM 提供的 UTC 时区日期时间类型
    pub create_time: Option<DateTimeUtc>,
}

/// 实体关系定义（当前无关联关系）
// Copy - 支持按位复制（枚举体型）
// Clone - 支持克隆
// Debug - 支持调试输出
// EnumIter - SeaORM 宏，生成枚举迭代器
// DeriveRelation - SeaORM 宏，自动生成 RelationTrait 实现
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// 空枚举表示该实体当前没有定义与其他实体的关联关系
// 虽然 stock、in_order、out_order 等表通过 warehouse_id 逻辑引用此表，但 SeaORM 层面未声明反向关联
pub enum Relation {}

/// ActiveModel 行为实现
// ActiveModelBehavior 是 SeaORM 的 trait，为空实现（默认行为）
// 提供 insert()、update()、delete() 等数据库操作方法的基础支持
impl ActiveModelBehavior for ActiveModel {}
