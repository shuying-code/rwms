// ============================================
// 供应商实体模块
// 对应数据库表: supplier
// ============================================

// 导入 SeaORM 框架核心类型，包含 EntityTrait、DeriveEntityModel、ActiveModelBehavior 等
use sea_orm::entity::prelude::*;
// 导入 serde 序列化/反序列化 trait，用于 JSON 交互
use serde::{Deserialize, Serialize};

/// 供应商实体模型
///
/// # 字段说明
/// * `supplier_id` - 供应商ID，主键，自增
/// * `supplier_name` - 供应商名称
/// * `contact` - 联系人姓名
/// * `phone` - 联系电话
/// * `remark` - 备注信息
// Clone - 支持克隆，便于在多处使用同一供应商数据
// Debug - 支持调试输出，方便日志打印和调试
// PartialEq - 支持相等比较，用于测试断言
// DeriveEntityModel - SeaORM 宏，自动生成 Entity、Column、PrimaryKey 等关联类型
// Serialize - 支持 serde 序列化，可将结构体转为 JSON 等格式（如 API 返回供应商列表）
// Deserialize - 支持 serde 反序列化，可将 JSON 等格式解析为结构体（如前端提交供应商数据）
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
// 指定该实体映射到数据库中的 "supplier" 表
#[sea_orm(table_name = "supplier")]
pub struct Model {
    /// 供应商ID，主键
    // sea_orm(primary_key) 标记此字段为主键，默认 auto_increment = true（自增）
    #[sea_orm(primary_key)]
    // i32 类型对应数据库中的 INT，作为供应商唯一标识，由数据库自动生成
    pub supplier_id: i32,
    /// 供应商名称
    // String 类型对应数据库中的 VARCHAR，存储供应商名称，为必填字段
    pub supplier_name: String,
    /// 联系人姓名
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，联系人可为空
    pub contact: Option<String>,
    /// 联系电话
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，联系电话可为空
    pub phone: Option<String>,
    /// 备注信息
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
// 虽然 in_order 表通过 supplier_id 逻辑引用此表，但 SeaORM 层面未声明反向关联
pub enum Relation {}

/// ActiveModel 行为实现
// ActiveModelBehavior 是 SeaORM 的 trait，为空实现（默认行为）
// 提供 insert()、update()、delete() 等数据库操作方法的基础支持
impl ActiveModelBehavior for ActiveModel {}
