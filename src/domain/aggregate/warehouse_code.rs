// ============================================
// 仓库编码值对象（Value Object）
// DDD 值对象：不可变，含格式验证
// 封装仓库编码的业务标识语义
// ============================================

/// 仓库编码值对象
/// 不可变：创建后不可修改，修改需替换
#[derive(Debug, Clone, PartialEq, Eq)] // Debug=调试, Clone=克隆, PartialEq/Eq=值相等
pub struct WarehouseCode(String); // 内部存储验证后的编码

impl WarehouseCode {
    /// 创建仓库编码值对象
    /// 规则：非空，非纯空白
    /// 验证失败返回错误
    pub fn new(value: String) -> Result<Self, String> {
        let trimmed = value.trim(); // 去除首尾空白
        if trimmed.is_empty() {     // 空编码校验
            return Err("仓库编码不能为空".to_string()); // 业务错误
        }
        Ok(Self(trimmed.to_string())) // 验证通过，创建值对象
    }

    /// 获取编码字符串
    /// 返回内部存储的已验证编码
    pub fn as_str(&self) -> &str {
        &self.0 // 返回内部字符串引用
    }
}

impl std::fmt::Display for WarehouseCode {
    /// 格式化输出：用于日志
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0) // 输出编码
    }
}

impl serde::Serialize for WarehouseCode {
    /// JSON 序列化：输出为字符串
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer) // 委托 String 的序列化
    }
}
