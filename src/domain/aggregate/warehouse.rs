// ============================================
// 仓库领域实体（富领域模型）
// DDD 聚合根（Aggregate Root）
// 封装仓库容量管理的不变式：
//   0 <= remain_space <= max_space
//   已用空间 = max_space - remain_space
// ============================================

use chrono::{DateTime, Utc}; // 时间类型
use serde::ser::{Serialize, Serializer, SerializeStruct}; // 序列化
use crate::domain::{DomainError, DomainResult}; // 领域错误类型
use super::warehouse_code::WarehouseCode; // 仓库编码值对象

/// DDD 聚合根 - 仓库领域实体
/// 维护容量不变式，对外暴露业务行为而非 setter
#[derive(Clone, Debug)]
pub struct Warehouse {
    warehouse_id: String,             // 仓库编号（业务主键）
    warehouse_code: WarehouseCode,    // 仓库编码（值对象，含验证）
    manager: Option<String>,          // 仓库管理员姓名（可选）
    max_space: i32,                   // 最大容量（私有，通过 adjust_capacity 修改）
    remain_space: i32,               // 剩余容量（私有，通过 allocate/release 修改）
    create_time: Option<DateTime<Utc>>, // 创建时间
}

impl Warehouse {
    /// 工厂方法：创建新仓库
    /// 自动设置 remain_space = max_space，create_time = 当前时间
    /// 业务不变式：新仓库的剩余空间 = 最大容量
    pub fn new(
        warehouse_id: String,           // 仓库编号（业务主键）
        warehouse_code: WarehouseCode,  // 仓库编码（值对象，已通过验证）
        manager: Option<String>,        // 管理员姓名（可选）
        max_space: i32,                 // 最大容量
    ) -> DomainResult<Self> {
        if max_space <= 0 {             // 最大容量必须为正数
            return Err(DomainError::InvalidOperation("最大容量必须大于0".to_string()));
        }
        Ok(Self {
            warehouse_id,               // 仓库编号
            warehouse_code,             // 仓库编码
            manager,                    // 管理员
            max_space,                  // 最大容量
            remain_space: max_space,    // 新仓库剩余空间 = 最大容量（空仓库）
            create_time: Some(Utc::now()), // 创建时间 = 当前 UTC 时间
        })
    }

    /// 从仓储重建聚合根（用于从数据库加载已有数据）
    /// 不走工厂方法，因为已有数据已经满足业务约束
    pub fn reconstitute(
        warehouse_id: String,             // 仓库编号
        warehouse_code: WarehouseCode,    // 仓库编码
        manager: Option<String>,          // 管理员姓名
        max_space: i32,                   // 最大容量
        remain_space: i32,                // 剩余容量
        create_time: Option<DateTime<Utc>>, // 创建时间
    ) -> Self {
        Self {
            warehouse_id,                 // 仓库编号
            warehouse_code,               // 仓库编码
            manager,                      // 管理员
            max_space,                    // 最大容量
            remain_space,                 // 剩余容量（数据库中已有值）
            create_time,                  // 创建时间
        }
    }

    // === Getter 方法（只读访问，保持封装） ===

    /// 获取最大容量（只读）
    pub fn max_space(&self) -> i32 {
        self.max_space                    // 最大容量（件）
    }

    /// 获取剩余容量（只读）
    pub fn remain_space(&self) -> i32 {
        self.remain_space                 // 剩余容量（件）
    }

    /// 获取仓库编号（只读）
    pub fn warehouse_id(&self) -> &str {
        &self.warehouse_id               // 仓库业务编号
    }

    /// 获取仓库编码（只读）
    pub fn warehouse_code(&self) -> &WarehouseCode {
        &self.warehouse_code             // 仓库编码值对象
    }

    /// 获取仓库管理员（只读）
    pub fn manager(&self) -> &Option<String> {
        &self.manager                    // 管理员姓名（可能为空）
    }

    /// 获取创建时间（只读）
    pub fn create_time(&self) -> &Option<DateTime<Utc>> {
        &self.create_time                // 创建时间（可能为空）
    }

    /// 入库占用仓库空间（领域行为）
    /// 不变式校验：quantity > 0 且 remain_space >= quantity
    pub fn allocate_space(&mut self, quantity: i32) -> DomainResult<()> {
        if quantity <= 0 {               // 占用数量必须为正数
            return Err(DomainError::InvalidOperation("占用数量必须大于0".to_string()));
        }
        if self.remain_space < quantity { // 剩余空间不足
            return Err(DomainError::BusinessRuleViolation(
                format!("仓库剩余空间不足：剩余 {}，需要占用 {}", self.remain_space, quantity)
            ));
        }
        self.remain_space -= quantity;   // 扣减剩余空间（维护不变式）
        Ok(())
    }

    /// 出库释放仓库空间（领域行为）
    /// 不变式校验：quantity > 0，释放后不超过 max_space
    pub fn release_space(&mut self, quantity: i32) -> DomainResult<()> {
        if quantity <= 0 {               // 释放数量必须为正数
            return Err(DomainError::InvalidOperation("释放数量必须大于0".to_string()));
        }
        self.remain_space = (self.remain_space + quantity).min(self.max_space); // 增加但不超最大容量
        Ok(())
    }

    /// 更新仓库基本信息（领域行为）
    /// 只更新编码和管理员，不修改容量
    pub fn update_info(&mut self, warehouse_code: WarehouseCode, manager: Option<String>) -> DomainResult<()> {
        self.warehouse_code = warehouse_code; // 仓库编码（值对象，已通过验证）
        self.manager = manager;               // 管理员姓名
        Ok(())
    }

    /// 调整仓库最大容量（领域行为，保持容量不变式）
    /// 规则：新容量必须 >= 已用空间（已用 = max_space - remain_space）
    pub fn adjust_capacity(&mut self, max_space: i32) -> DomainResult<()> {
        if max_space <= 0 {              // 最大容量必须为正数
            return Err(DomainError::InvalidOperation("最大容量必须大于0".to_string()));
        }
        let used_space = self.max_space - self.remain_space; // 计算已用空间
        if used_space > max_space {      // 新容量装不下已用空间
            return Err(DomainError::BusinessRuleViolation(
                format!("新容量不足以容纳已用空间：已用 {}，新最大容量 {}", used_space, max_space)
            ));
        }
        self.max_space = max_space;       // 更新最大容量
        self.remain_space = max_space - used_space; // 重新计算剩余空间（保持不变式）
        Ok(())
    }
}

impl Serialize for Warehouse {
    /// JSON 序列化：将领域实体转为 API 响应格式
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Warehouse", 6)?; // 6 个字段
        s.serialize_field("warehouse_id", &self.warehouse_id)?;   // 仓库编号
        s.serialize_field("warehouse_code", &self.warehouse_code.as_str())?; // 仓库编码（字符串）
        s.serialize_field("manager", &self.manager)?;             // 管理员
        s.serialize_field("max_space", &self.max_space)?;         // 最大容量
        s.serialize_field("remain_space", &self.remain_space)?;   // 剩余容量
        s.serialize_field("create_time", &self.create_time)?;     // 创建时间
        s.end()
    }
}
