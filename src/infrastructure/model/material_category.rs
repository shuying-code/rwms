// ============================================
// 物资分类实体模块
// 对应数据库表: material_category
// 用于对物资进行分组管理（如：电子产品、办公用品、劳保用品等）
// ============================================

// 导入 SeaORM ORM 框架核心类型：EntityTrait、DeriveEntityModel 等宏
use sea_orm::entity::prelude::*;

// 导入 serde 序列化框架：Serialize（→ JSON）、Deserialize（← JSON）
use serde::{Deserialize, Serialize};

/// 物资分类实体模型
///
/// # 字段说明
/// * `category_id` - 分类ID（自增主键，数据库自动生成）
/// * `category_name` - 分类名称（唯一约束，不同分类不能同名）
/// * `remark` - 分类备注（可选，如分类说明、适用范围等）
///
/// # 使用场景
/// * 物资表（material）通过 category_id 外键引用此表
/// * 前端下拉框选择物资分类时从本表获取数据
// Clone - 支持克隆，便于在多处使用同一分类数据
// Debug - 支持调试输出，方便日志打印
// PartialEq - 支持相等比较，用于测试断言
// DeriveEntityModel - SeaORM 宏，自动生成 Entity、Column、PrimaryKey 等关联类型
// Serialize - 支持 serde 序列化，可将结构体转为 JSON 等格式（如 API 返回分类列表）
// Deserialize - 支持 serde 反序列化，可将 JSON 等格式解析为结构体（如前端提交分类数据）
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
// 指定该实体映射到数据库中的 "material_category" 表
#[sea_orm(table_name = "material_category")]
pub struct Model {
    /// 分类ID（自增主键，auto_increment = true）
    // sea_orm(primary_key) 标记此字段为主键，默认 auto_increment = true（自增）
    #[sea_orm(primary_key)]
    // i32 类型对应数据库中的 INT，作为分类唯一标识，由数据库自动生成
    pub category_id: i32,
    /// 分类名称（必填，唯一）
    // String 类型对应数据库中的 VARCHAR，存储分类名称
    // 应在数据库层面设置唯一约束，确保不同分类不会重名
    pub category_name: String,
    /// 备注信息（可选）
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，备注信息可为空
    // 可用于记录分类说明、适用范围、分类编码规则等补充信息
    pub remark: Option<String>,
}

/// 实体关系定义（当前无关联关系，暂无外键关联）
// Copy - 支持按位复制（枚举体型，大小固定，可安全 Copy）
// Clone - 支持克隆
// Debug - 支持调试输出
// EnumIter - SeaORM 宏，生成枚举迭代器，用于遍历所有关系
// DeriveRelation - SeaORM 宏，自动生成 RelationTrait 实现
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// 空枚举表示该实体当前没有定义与其他实体的关联关系
// 虽然 material 表通过 category_id 逻辑引用此表，但 SeaORM 层面未声明反向关联
pub enum Relation {}

/// ActiveModel 行为实现：提供 insert()、update()、delete() 等方法
// ActiveModelBehavior 是 SeaORM 的 trait，为空实现（默认行为）
// 如需自定义钩子逻辑（如插入前校验分类名唯一性），可在此 impl 中覆写 before_insert 等方法
impl ActiveModelBehavior for ActiveModel {}
