// ============================================
// 订单类型值对象（Value Object）
// DDD 值对象：不可变，通过枚举约束有效值
// 区分入库单和出库单
// ============================================

use serde::{Deserialize, Serialize}; // JSON 序列化支持

/// 订单类型枚举
/// 不可变值对象，新类型通过克隆或常量获取
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)] // Debug=调试打印, Clone=显式克隆, Copy=栈复制, PartialEq/Eq=相等比较
pub enum OrderType {
    /// 入库单类型
    #[serde(rename = "入库")] // JSON 序列化为"入库"
    Inbound,                  // 入库
    /// 出库单类型
    #[serde(rename = "出库")] // JSON 序列化为"出库"
    Outbound,                 // 出库
}

impl OrderType {
    /// 转为业务展示字符串
    /// 返回中文标签，用于前端展示
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Inbound => "入库",  // 入库单业务展示
            Self::Outbound => "出库", // 出库单业务展示
        }
    }

    /// 从字符串创建订单类型
    /// 不区分大小写匹配
    /// 无法匹配时返回 None
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() { // 转为小写进行不区分大小写匹配
            "入库" | "inbound" => Some(Self::Inbound),   // 匹配入库（中英文）
            "出库" | "outbound" => Some(Self::Outbound),  // 匹配出库（中英文）
            _ => None, // 无法识别的类型返回 None
        }
    }
}

impl std::fmt::Display for OrderType {
    /// 格式化输出：用于日志和调试
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str()) // 委托 as_str() 输出中文标签
    }
}
