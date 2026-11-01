// ============================================
// 计量单位值对象（Value Object）
// DDD 值对象：不可变，约束有效单位范围
// 封装物资计量单位的业务语义
// ============================================

/// 计量单位值对象
/// 不可变：创建后不可修改
/// 当前接受自由文本（后续可约束为枚举）
#[derive(Debug, Clone, PartialEq, Eq)] // Debug=调试, Clone=克隆, PartialEq/Eq=值相等
pub struct Unit(String); // 内部存储验证后的单位字符串

impl Unit {
    /// 创建计量单位值对象
    /// 规则：非空，非纯空白
    /// 验证失败返回错误
    pub fn new(value: String) -> Result<Self, String> {
        let trimmed = value.trim(); // 去除首尾空白
        if trimmed.is_empty() {     // 空单位校验
            return Err("计量单位不能为空".to_string()); // 业务错误
        }
        Ok(Self(trimmed.to_string())) // 验证通过，创建值对象
    }

    /// 获取单位字符串
    /// 返回内部存储的已验证单位
    pub fn as_str(&self) -> &str {
        &self.0 // 返回内部字符串引用
    }
}

impl std::fmt::Display for Unit {
    /// 格式化输出：用于日志
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0) // 输出单位
    }
}

impl serde::Serialize for Unit {
    /// JSON 序列化：输出为字符串
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer) // 委托 String 的序列化
    }
}
