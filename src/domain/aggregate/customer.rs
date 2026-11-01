// ============================================
// 客户领域实体
// DDD 聚合根（Aggregate Root）
// 简单 CRUD 实体，包含名称非空验证
// ============================================

use serde::ser::{Serialize, Serializer, SerializeStruct}; // 序列化支持
use crate::domain::{DomainError, DomainResult}; // 领域错误类型
use super::contact::Contact; // 联系人值对象
use super::phone::Phone; // 电话号码值对象

/// DDD 聚合根 - 客户领域实体
#[derive(Clone, Debug)]
pub struct Customer {
    customer_id: i32,          // 客户编号（数据库自增主键）
    customer_name: String,     // 客户名称（业务标识）
    contact: Option<Contact>,  // 联系人（值对象，可选）
    phone: Option<Phone>,      // 联系电话（值对象，可选）
    remark: Option<String>,    // 备注（可选，自由文本）
}

impl Customer {
    /// 工厂方法：创建新客户
    /// customer_id 设为 0（由数据库自增生成）
    pub fn new(
        customer_name: String,    // 客户名称
        contact: Option<Contact>, // 联系人（值对象）
        phone: Option<Phone>,     // 联系电话（值对象）
        remark: Option<String>,   // 备注
    ) -> Self {
        Self {
            customer_id: 0,       // 新记录 ID 由数据库生成
            customer_name,        // 客户名称
            contact,              // 联系人
            phone,                // 联系电话
            remark,               // 备注
        }
    }

    /// 从仓储重建客户（已有数据，不做额外校验）
    pub fn reconstitute(
        customer_id: i32,         // 客户编号
        customer_name: String,    // 客户名称
        contact: Option<Contact>, // 联系人
        phone: Option<Phone>,     // 联系电话
        remark: Option<String>,   // 备注
    ) -> Self {
        Self {
            customer_id,          // 已有 ID
            customer_name,        // 客户名称
            contact,              // 联系人
            phone,                // 联系电话
            remark,               // 备注
        }
    }

    // === Getter 方法（只读访问，保持封装） ===

    /// 获取客户编号（只读）
    pub fn customer_id(&self) -> i32 {
        self.customer_id           // 自增主键
    }

    /// 获取客户名称（只读）
    pub fn customer_name(&self) -> &str {
        &self.customer_name        // 客户名称
    }

    /// 获取联系人（只读）
    pub fn contact(&self) -> &Option<Contact> {
        &self.contact              // 联系人值对象（可能为空）
    }

    /// 获取联系电话（只读）
    pub fn phone(&self) -> &Option<Phone> {
        &self.phone                // 电话号码值对象（可能为空）
    }

    /// 获取备注（只读）
    pub fn remark(&self) -> &Option<String> {
        &self.remark               // 备注文本（可能为空）
    }

    /// 更新客户信息（领域行为）
    /// 校验名称非空
    pub fn update(
        &mut self,
        customer_name: String,    // 新客户名称
        contact: Option<Contact>, // 新联系人
        phone: Option<Phone>,     // 新联系电话
        remark: Option<String>,   // 新备注
    ) -> DomainResult<()> {
        if customer_name.trim().is_empty() { // 名称不能为空
            return Err(DomainError::InvalidOperation("客户名称不能为空".to_string()));
        }
        self.customer_name = customer_name; // 更新名称
        self.contact = contact;             // 更新联系人
        self.phone = phone;                 // 更新电话
        self.remark = remark;               // 更新备注
        Ok(())
    }
}

impl Serialize for Customer {
    /// JSON 序列化：领域实体 → API 响应
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Customer", 5)?; // 5 个字段
        s.serialize_field("customer_id", &self.customer_id)?;   // 客户编号
        s.serialize_field("customer_name", &self.customer_name)?; // 客户名称
        s.serialize_field("contact", &self.contact)?;           // 联系人
        s.serialize_field("phone", &self.phone)?;               // 联系电话
        s.serialize_field("remark", &self.remark)?;             // 备注
        s.end()
    }
}
