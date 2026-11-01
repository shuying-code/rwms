// ============================================
// 客户实体 - 对应数据库 customer 表
// 记录客户信息，用于出库单业务
// ============================================

// 导入 SeaORM 框架核心类型，包含 EntityTrait、DeriveEntityModel、ActiveModelBehavior 等宏和 trait
use sea_orm::entity::prelude::*;
// 导入 serde 序列化/反序列化 trait，用于 JSON 序列化（API 响应）和反序列化（请求解析）
use serde::{Deserialize, Serialize};

/// 客户实体模型
///
/// # 字段说明
/// * `customer_id` - 客户ID，主键，自增
/// * `customer_name` - 客户名称
/// * `contact` - 联系人（可选）
/// * `phone` - 联系电话（可选）
/// * `remark` - 备注（可选）
// Clone - 支持克隆，便于在多处使用同一客户数据
// Debug - 支持调试输出，方便日志打印和调试
// PartialEq - 支持相等比较，用于测试断言
// DeriveEntityModel - SeaORM 宏，自动生成 Entity、Column、PrimaryKey 等关联类型
// Serialize - 支持 serde 序列化，可将结构体转为 JSON 等格式
// Deserialize - 支持 serde 反序列化，可将 JSON 等格式解析为结构体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
// 指定该实体映射到数据库中的 "customer" 表
#[sea_orm(table_name = "customer")]
pub struct Model {
    /// 客户ID，主键
    // sea_orm(primary_key) 标记此字段为主键，默认 auto_increment = true（自增）
    #[sea_orm(primary_key)]
    // i32 类型对应数据库中的 INT，作为客户唯一标识
    pub customer_id: i32,
    /// 客户名称
    // String 类型对应数据库中的 VARCHAR，存储客户名称，为必填字段
    pub customer_name: String,
    /// 联系人
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，联系人可为空
    pub contact: Option<String>,
    /// 联系电话
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，联系电话可为空
    pub phone: Option<String>,
    /// 备注
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，备注信息可为空
    pub remark: Option<String>,
}

/// 实体关系定义（当前无关联关系）
// Copy - 支持按位复制（枚举体型，大小固定，可安全 Copy）
// Clone - 支持克隆
// Debug - 支持调试输出
// EnumIter - SeaORM 宏，生成枚举迭代器，用于遍历所有关系
// DeriveRelation - SeaORM 宏，自动生成 RelationTrait 实现
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// 空枚举表示该实体当前没有定义与其他实体的关联关系
// 若需关联，可添加如 HasOne、HasMany 等变体
pub enum Relation {}

/// ActiveModel 行为实现
// ActiveModelBehavior 是 SeaORM 的 trait，为空实现（默认行为）
// 提供 insert()、update()、delete() 等数据库操作方法的基础支持
// 如需自定义钩子逻辑（如插入前自动填充时间戳），可在此 impl 中覆写方法
impl ActiveModelBehavior for ActiveModel {}
