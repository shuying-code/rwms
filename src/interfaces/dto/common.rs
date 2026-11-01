// ============================================
// 通用 DTO 类型
// 包含响应包装器、分页参数等共用结构
// 错误类型见 error.rs
// ============================================

use serde::{Deserialize, Serialize}; // 导入 Serde 序列化/反序列化框架
use super::error::ServiceError; // 导入统一业务错误类型，用于分页参数校验

// ============================================
// 通用响应包装器
// 所有 API 响应统一使用此结构，前端通过 result.code 判断成功/失败
// ============================================

/// 通用 API 响应结构
///
/// # 字段说明
/// * `code` - 业务状态码，200=成功，其他值为业务错误（如 400/401/404/500）
/// * `msg` - 人类可读的错误描述（中文）
/// * `data` - 业务数据载荷，成功时有值，失败时为 null
///
/// # 设计说明
/// 使用泛型 T 使得 data 可以是任意类型：
/// * T = Vec<Material> 时：列表接口
/// * T = Material 时：详情接口
/// * T = () 时：操作类接口（无返回数据）
/// * T = LoginResponse 时：登录成功响应
///
/// # JSON 示例
/// 成功：{"code":200,"msg":"成功","data":[...]}
/// 失败：{"code":404,"msg":"物资不存在","data":null}
#[derive(Debug, Serialize, Deserialize)] // Debug 便于调试，Serialize 用于序列化 JSON 响应，Deserialize 支持测试
pub struct ApiResponse<T> {
    /// 业务状态码（200=成功）
    pub code: i32, // 业务状态码，200 表示成功，400/401/404/409/500 等表示各类错误
    /// 消息文本（成功时通常为"成功"，失败时为具体错误原因）
    pub msg: String, // 中文消息文本，前端可直接展示给用户
    /// 业务数据，为 None 时 serde 自动跳过序列化（skip_serializing_if = "Option::is_none"）
    #[serde(skip_serializing_if = "Option::is_none")] // 当 data 为 None 时不在 JSON 中输出 data 字段
    pub data: Option<T>, // 业务数据载荷，使用 Option 包装，失败时为 None
}

impl<T> ApiResponse<T> {
    /// 构造成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 200, // 成功状态码
            msg: "成功".to_string(), // 默认成功消息
            data: Some(data), // 包装业务数据
        }
    }

    /// 构造错误响应
    pub fn error(code: i32, msg: String) -> Self {
        Self {
            code, // 传入的错误码（如 400/401/404/409/500）
            msg, // 传入的错误描述信息
            data: None, // 错误响应不包含业务数据
        }
    }

    /// 构造未认证响应（401）
    #[allow(dead_code)] // 允许未使用警告，此方法供后续扩展使用
    pub fn unauthorized() -> Self {
        Self {
            code: 401, // HTTP 401 未认证状态码
            msg: "未登录".to_string(), // 未认证提示消息
            data: None, // 无业务数据
        }
    }
}

// ============================================
// 分页查询相关 DTO
// ============================================

/// 分页查询参数（物资、分类、仓库、供应商、客户等通用）
///
/// # 字段说明
/// * `page_index` - 页码（从 1 开始），默认 1
/// * `page_size` - 每页记录数，默认 10，最大 100
///
/// # 使用说明
/// 查询第 N 页：page_index = N, page_size = 每页数量
/// SQL 等价：LIMIT page_size OFFSET (page_index - 1) * page_size
#[derive(Debug, Deserialize)] // Debug 便于调试打印，Deserialize 用于从查询参数解析
pub struct PageQuery {
    #[serde(default = "default_page_index")] // 前端未传时使用默认值 1
    pub page_index: i32, // 当前页码，从 1 开始（转换为 OFFSET 时需减 1）
    #[serde(default = "default_page_size")] // 前端未传时使用默认值 10
    pub page_size: i32, // 每页记录数，最大 100，防止一次查询过多数据
}

impl PageQuery {
    /// 校验分页参数
    pub fn validate(&self) -> Result<(), ServiceError> {
        if self.page_index < 1 { // 页码不能小于 1
            return Err(ServiceError::Validation("page_index 必须 >= 1".to_string()));
        }
        if self.page_size < 1 { // 每页条数不能小于 1
            return Err(ServiceError::Validation("page_size 必须 >= 1".to_string()));
        }
        if self.page_size > 100 { // 每页条数不能超过 100，防止一次返回过多数据
            return Err(ServiceError::Validation("page_size 不能超过 100".to_string()));
        }
        Ok(()) // 校验通过
    }
}

/// 默认页码（从第 1 页开始）
pub(crate) fn default_page_index() -> i32 { // pub(crate) 表示仅在当前 crate 内可见
    1 // 默认第一页
}

/// 默认每页条数（10 条，避免一次返回过多数据）
pub(crate) fn default_page_size() -> i32 { // pub(crate) 表示仅在当前 crate 内可见
    10 // 默认每页 10 条
}
