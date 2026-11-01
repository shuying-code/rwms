// ============================================
// 用户角色值对象（Value Object）
// DDD 值对象：不可变枚举，替代原始 String 类型
// 封装角色层级和权限语义
// ============================================

use serde::{Deserialize, Serialize}; // JSON 序列化支持

/// 用户角色枚举
/// 替代原 `role: String` 自由字符串，提供类型安全
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)] // Debug=调试, Clone=克隆, PartialEq/Eq=相等比较
pub enum UserRole {
    /// 超级管理员（全部权限：用户管理、系统配置）
    #[serde(rename = "超级管理员")] // 序列化为中文角色名
    SuperAdmin,                    // 超级管理员
    /// 仓库管理员（业务权限：出入库审核、库存管理）
    #[serde(rename = "仓库管理员")]
    WarehouseAdmin,                // 仓库管理员
    /// 操作员（基本权限：出入库创建、基础 CRUD）
    #[serde(rename = "操作员")]
    Operator,                      // 操作员
}

impl UserRole {
    /// 转为业务展示字符串
    /// 返回角色中文名称，用于前端显示
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SuperAdmin => "超级管理员", // 最高权限角色
            Self::WarehouseAdmin => "仓库管理员", // 业务管理角色
            Self::Operator => "操作员", // 日常操作角色
        }
    }

    /// 从字符串创建角色
    /// 无法匹配时返回 None
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "超级管理员" => Some(Self::SuperAdmin),    // 匹配超级管理员
            "仓库管理员" => Some(Self::WarehouseAdmin), // 匹配仓库管理员
            "操作员" => Some(Self::Operator),          // 匹配操作员
            _ => None, // 无法识别的角色返回 None
        }
    }
}

impl std::fmt::Display for UserRole {
    /// 格式化输出：用于日志和数据库存储
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str()) // 委托 as_str() 输出中文
    }
}
