// ============================================
// 账号状态值对象（Value Object）
// DDD 值对象：不可变枚举，替代原始 i8 类型
// 封装"启用/禁用"的业务语义
// ============================================

use serde::{Deserialize, Serialize}; // JSON 序列化支持

/// 账号状态枚举
/// 替代原 `status: Option<i8>` 数据库原始类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)] // Debug=调试, Clone=克隆, Copy=栈复制, PartialEq/Eq=比较
pub enum AccountStatus {
    /// 账号已启用（可登录）
    #[serde(rename = "1")]   // 序列化为 "1"
    Active,                  // 活跃状态
    /// 账号已禁用（不可登录）
    #[serde(rename = "0")]   // 序列化为 "0"
    Disabled,                // 禁用状态
}

impl AccountStatus {
    /// 转为数据库存储的整数值
    /// Active → 1, Disabled → 0
    #[allow(dead_code)]
    pub fn as_i8(&self) -> i8 {
        match self {
            Self::Active => 1,   // 启用=1
            Self::Disabled => 0, // 禁用=0
        }
    }

    /// 从数据库整数值重建
    /// 未知值默认为 Disabled（安全策略）
    pub fn from_i8(v: i8) -> Self {
        match v {
            1 => Self::Active,    // 1=启用
            0 => Self::Disabled,  // 0=禁用
            _ => Self::Disabled,  // 未知值保守处理为禁用
        }
    }

    /// 判断是否可登录
    /// 只有 Active 状态允许登录
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active) // 匹配 Active 状态
    }
}

impl std::fmt::Display for AccountStatus {
    /// 格式化输出：用于日志
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "启用"),   // 启用状态
            Self::Disabled => write!(f, "禁用"), // 禁用状态
        }
    }
}
