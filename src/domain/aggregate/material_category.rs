// ============================================
// 物资分类领域实体
// DDD 聚合根（Aggregate Root）
// 简单 CRUD 实体，包含分类名称非空验证
// ============================================

use serde::ser::{Serialize, Serializer, SerializeStruct}; // 序列化支持
use crate::domain::{DomainError, DomainResult}; // 领域错误类型

/// DDD 聚合根 - 物资分类领域实体
#[derive(Clone, Debug)]
pub struct MaterialCategory {
    category_id: i32,          // 分类编号（数据库自增主键）
    category_name: String,     // 分类名称（业务标识，唯一）
    remark: Option<String>,    // 备注（可选，自由文本）
}

impl MaterialCategory {
    /// 工厂方法：创建新分类
    /// category_id 设为 0（由数据库自增生成）
    pub fn new(
        category_name: String,   // 分类名称
        remark: Option<String>,  // 备注
    ) -> Self {
        Self {
            category_id: 0,      // 新记录 ID 由数据库生成
            category_name,       // 分类名称
            remark,              // 备注
        }
    }

    /// 从仓储重建分类（已有数据，不做额外校验）
    pub fn reconstitute(
        category_id: i32,        // 分类编号
        category_name: String,   // 分类名称
        remark: Option<String>,  // 备注
    ) -> Self {
        Self {
            category_id,         // 已有 ID
            category_name,       // 分类名称
            remark,              // 备注
        }
    }

    // === Getter 方法（只读访问，保持封装） ===

    /// 获取分类编号（只读）
    pub fn category_id(&self) -> i32 {
        self.category_id          // 自增主键
    }

    /// 获取分类名称（只读）
    pub fn category_name(&self) -> &str {
        &self.category_name       // 分类名称
    }

    /// 获取备注（只读）
    pub fn remark(&self) -> &Option<String> {
        &self.remark              // 备注文本
    }

    /// 更新分类信息（领域行为）
    /// 校验分类名称非空
    pub fn update(
        &mut self,
        category_name: String,   // 新分类名称
        remark: Option<String>,  // 新备注
    ) -> DomainResult<()> {
        if category_name.trim().is_empty() { // 分类名称不能为空
            return Err(DomainError::InvalidOperation("分类名称不能为空".to_string()));
        }
        self.category_name = category_name; // 更新名称
        self.remark = remark;               // 更新备注
        Ok(())
    }
}

impl Serialize for MaterialCategory {
    /// JSON 序列化：领域实体 → API 响应
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("MaterialCategory", 3)?; // 3 个字段
        s.serialize_field("category_id", &self.category_id)?;   // 分类编号
        s.serialize_field("category_name", &self.category_name)?; // 分类名称
        s.serialize_field("remark", &self.remark)?;             // 备注
        s.end()
    }
}
