// ============================================
// 系统用户实体模块
// 对应数据库表 sys_user，用于存储系统用户信息
// ============================================

// 导入 SeaORM 框架核心类型，包含 EntityTrait、DeriveEntityModel、ActiveModelBehavior、DateTimeUtc 等
use sea_orm::entity::prelude::*;
// 导入 serde 序列化/反序列化 trait，用于 JSON 交互
use serde::{Deserialize, Serialize};

/// 系统用户实体模型
///
/// # 字段说明
/// * `user_id` - 用户唯一标识（主键）
/// * `username` - 登录用户名
/// * `password` - 登录密码（应加密存储）
/// * `real_name` - 真实姓名
/// * `role` - 用户角色（如：超级管理员、普通用户）
/// * `phone` - 联系电话
/// * `token` - 当前登录令牌（用于单点登录控制）
/// * `create_time` - 账号创建时间
/// * `status` - 账号状态（0=禁用，1=启用）
// Clone - 支持克隆，便于在多处使用同一用户数据
// Debug - 支持调试输出，方便日志打印
// PartialEq - 支持相等比较，用于测试断言
// DeriveEntityModel - SeaORM 宏，自动生成 Entity、Column、PrimaryKey 等关联类型
// Serialize - 支持 serde 序列化，可将结构体转为 JSON 等格式
// Deserialize - 支持 serde 反序列化，可将 JSON 等格式解析为结构体
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
// 指定该实体映射到数据库中的 "sys_user" 表
#[sea_orm(table_name = "sys_user")]
pub struct Model {
    /// 用户ID（主键，非自增）
    // primary_key - 标记此字段为主键
    // auto_increment = false - 主键不由数据库自增，而是由业务层生成（如 UUID 或自定义编码）
    #[sea_orm(primary_key, auto_increment = false)]
    // String 类型对应数据库中的 VARCHAR，存储用户唯一标识
    pub user_id: String,
    /// 用户名（登录账号）
    // String 类型对应数据库中的 VARCHAR，存储登录用户名，为必填字段
    // 应设置唯一约束，确保用户名不重复
    pub username: String,
    /// 密码（建议加密存储）
    // String 类型对应数据库中的 VARCHAR，存储登录密码
    // 安全建议：密码应使用 bcrypt/argon2 等算法加密存储，严禁明文保存
    // 注意：序列化为 JSON 时可能需要特殊处理，避免密码泄露到前端
    pub password: String,
    /// 真实姓名
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，真实姓名可为空
    pub real_name: Option<String>,
    /// 用户角色
    // String 类型对应数据库中的 VARCHAR，存储用户角色标识，必填字段
    // 典型值："超级管理员"、"普通用户" 等，用于权限控制和菜单过滤
    pub role: String,
    /// 联系电话
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，联系电话可为空
    pub phone: Option<String>,
    /// JWT Token（用于会话管理）
    // Option<String> 对应数据库中的 VARCHAR NULLABLE，存储当前有效的 JWT 令牌
    // 用于单点登录控制：登录时生成 token 并存储，验证时比对请求中的 token 与数据库中的一致
    // 用户重新登录时 token 会更新，旧 token 自动失效
    pub token: Option<String>,
    /// 创建时间
    // Option<DateTimeUtc> 对应数据库中的 TIMESTAMP NULLABLE，存储账号创建时间
    // DateTimeUtc 是 SeaORM 提供的 UTC 时区日期时间类型
    pub create_time: Option<DateTimeUtc>,
    /// 账号状态：0-禁用，1-启用
    // Option<i8> 对应数据库中的 TINYINT NULLABLE，存储账号启用/禁用状态
    // 0 = 禁用（账号被冻结，无法登录），1 = 启用（正常状态）
    // 为 Option 类型是为了兼容历史数据中尚未设置状态的记录
    pub status: Option<i8>,
}

/// 实体关系定义（当前无关联关系）
// Copy - 支持按位复制（枚举体型）
// Clone - 支持克隆
// Debug - 支持调试输出
// EnumIter - SeaORM 宏，生成枚举迭代器
// DeriveRelation - SeaORM 宏，自动生成 RelationTrait 实现
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// 空枚举表示该实体当前没有定义与其他实体的关联关系
// 虽然 in_order、out_order 等表通过 operator_id 逻辑引用此表，但 SeaORM 层面未声明反向关联
pub enum Relation {}

/// ActiveModel 行为实现
// ActiveModelBehavior 是 SeaORM 的 trait，为空实现（默认行为）
// 提供 insert()、update()、delete() 等数据库操作方法的基础支持
impl ActiveModelBehavior for ActiveModel {}
