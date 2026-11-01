// ============================================
// 订单状态值对象（Value Object）
// DDD 值对象：不可变枚举，封装状态转换语义
// 序列化为语言无关 key（pending/approved/rejected）
// 反序列化兼容中英文（向后兼容旧数据）
// ============================================

use std::fmt; // 格式化 trait

/// DDD 值对象 - 订单状态枚举
/// 状态转换规则：Pending → Approved（审核通过）| Pending → Rejected（审核拒绝）
#[derive(Clone, Debug, PartialEq)] // Clone=可克隆, Debug=调试打印, PartialEq=相等比较
pub enum OrderStatus {
    /// 待审核（初始状态，可修改明细）
    Pending,                        // 待审核
    /// 审核通过（终态，不可修改）
    Approved,                       // 审核通过
    /// 审核拒绝（终态，不可修改）
    Rejected,                       // 审核不通过
}

impl OrderStatus {
    /// 从字符串解析订单状态
    /// 兼容语言无关 key 和中文标签（向后兼容旧数据）
    /// 无法匹配时返回 None
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" | "待审核" => Some(Self::Pending),     // 待审核（中英文均支持）
            "approved" | "审核通过" => Some(Self::Approved),  // 审核通过（中英文均支持）
            "rejected" | "审核不通过" => Some(Self::Rejected), // 审核拒绝（中英文均支持）
            _ => None, // 无法识别的状态返回 None
        }
    }

    /// 转换为中文展示字符串
    /// 用于日志输出和前端显示
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "待审核",     // Pending 的中文标签
            Self::Approved => "审核通过",   // Approved 的中文标签
            Self::Rejected => "审核不通过", // Rejected 的中文标签
        }
    }

    /// 转换为语言无关 key
    /// 用于 JSON 序列化和数据库存储（新数据统一用此格式）
    pub fn as_key(&self) -> &'static str {
        match self {
            Self::Pending => "pending",     // Pending 的语言无关 key
            Self::Approved => "approved",   // Approved 的语言无关 key
            Self::Rejected => "rejected",   // Rejected 的语言无关 key
        }
    }
}

impl fmt::Display for OrderStatus {
    /// 格式化输出：输出中文标签用于日志
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str()) // 委托 as_str() 输出中文
    }
}

impl serde::Serialize for OrderStatus {
    /// JSON 序列化：使用语言无关 key
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_key()) // 序列化为 "pending"/"approved"/"rejected"
    }
}

impl<'de> serde::Deserialize<'de> for OrderStatus {
    /// JSON 反序列化：从字符串解析（兼容中英文）
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?; // 先反序列化为字符串
        OrderStatus::from_str(&s)                   // 再从字符串解析（兼容中英文）
            .ok_or_else(|| serde::de::Error::custom(format!("未知订单状态: {}", s))) // 解析失败返回错误
    }
}
