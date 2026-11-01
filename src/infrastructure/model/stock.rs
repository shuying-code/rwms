// ============================================
// 库存实体
// 对应数据库表: stock
// 描述: 记录物资在各仓库的库存信息
// ============================================

// 导入 SeaORM 框架核心类型，包含 EntityTrait、DeriveEntityModel、ActiveModelBehavior、DateTimeUtc 等
use sea_orm::entity::prelude::*;
// 导入 serde 序列化/反序列化 trait，用于 JSON 交互
use serde::{Deserialize, Serialize};

/// 库存实体模型
///
/// # 字段说明
/// * `stock_id` - 库存ID（主键，自增）
/// * `material_id` - 物资编号（关联物资表）
/// * `warehouse_id` - 仓库编号（关联仓库表）
/// * `stock_num` - 库存数量
/// * `in_time` - 入库时间（首次入库）
/// * `update_time` - 更新时间（最后变动）
// Clone - 支持克隆，便于在多处使用同一库存数据
// Debug - 支持调试输出，方便日志打印
// PartialEq - 支持相等比较，用于测试断言
// DeriveEntityModel - SeaORM 宏，自动生成 Entity、Column、PrimaryKey 等关联类型
// Serialize - 支持 serde 序列化，可将结构体转为 JSON 等格式
// Deserialize - 支持 serde 反序列化，可将 JSON 等格式解析为结构体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
// 指定该实体映射到数据库中的 "stock" 表
#[sea_orm(table_name = "stock")]
pub struct Model {
    /// 库存ID（主键，自增）
    // sea_orm(primary_key) 标记此字段为主键，默认 auto_increment = true（自增）
    #[sea_orm(primary_key)]
    // i32 类型对应数据库中的 INT，作为库存记录唯一标识，由数据库自动生成
    pub stock_id: i32,
    /// 物资编号（关联物资表）
    // String 类型，存储物资编号，逻辑外键关联 material 表的 material_id
    // 与 warehouse_id 组合可唯一确定某物资在某仓库的库存记录
    pub material_id: String,
    /// 仓库编号（关联仓库表）
    // String 类型，存储仓库编号，逻辑外键关联 warehouse 表的 warehouse_id
    // 同一物资可分布在不同仓库中，每条记录对应一个仓库的库存
    pub warehouse_id: String,
    /// 库存数量
    // i32 类型对应数据库中的 INT，存储当前仓库中该物资的库存数量
    // 入库时增加，出库时减少
    pub stock_num: i32,
    /// 入库时间（首次入库）
    // Option<DateTimeUtc> 对应数据库中的 TIMESTAMP NULLABLE，存储该库存记录的首次入库时间
    // DateTimeUtc 是 SeaORM 提供的 UTC 时区日期时间类型
    pub in_time: Option<DateTimeUtc>,
    /// 更新时间（最后变动）
    // Option<DateTimeUtc> 对应数据库中的 TIMESTAMP NULLABLE，存储库存最后变动的时间
    // 每次入库或出库操作导致 stock_num 变化时更新此字段
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
// 虽然逻辑上关联了 material、warehouse 等表，但 SeaORM 层面未声明外键关系
pub enum Relation {}

/// ActiveModel 行为实现
// ActiveModelBehavior 是 SeaORM 的 trait，为空实现（默认行为）
// 提供 insert()、update()、delete() 等数据库操作方法的基础支持
impl ActiveModelBehavior for ActiveModel {}
