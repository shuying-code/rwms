// ============================================
// 电话号码值对象（Value Object）
// DDD 值对象：不可变，含格式验证
// 封装电话号码的业务规则
// ============================================

/// 电话号码值对象
/// 不可变：创建后不能修改，修改需替换为新实例
#[derive(Debug, Clone, PartialEq, Eq)] // Debug=调试, Clone=克隆, PartialEq/Eq=值相等比较
pub struct Phone(String); // 内部存储验证后的号码字符串

impl Phone {
    /// 创建电话号码值对象
    /// 执行格式验证：非空 + 只含数字和连字符
    /// 验证失败返回错误
    pub fn new(value: String) -> Result<Self, String> {
        let trimmed = value.trim(); // 去除首尾空格
        if trimmed.is_empty() {     // 空字符串校验
            return Err("电话号码不能为空".to_string()); // 业务错误提示
        }
        if !trimmed.chars().all(|c| c.is_ascii_digit() || c == '-') { // 只允许数字和连字符
            return Err(format!("电话号码格式无效: {}", trimmed)); // 格式错误提示
        }
        Ok(Self(trimmed.to_string())) // 验证通过，创建值对象
    }

    /// 获取电话号码字符串
    /// 返回内部存储的已验证号码
    pub fn as_str(&self) -> &str {
        &self.0 // 返回内部字符串引用
    }
}

impl std::fmt::Display for Phone {
    /// 格式化输出：用于日志
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0) // 输出号码字符串
    }
}

impl serde::Serialize for Phone {
    /// JSON 序列化：输出为字符串
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer) // 委托 String 的序列化
    }
}
