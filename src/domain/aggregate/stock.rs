// ============================================
// 库存领域实体（富领域模型）
// DDD 聚合根（Aggregate Root）
// 包含入库增加/出库扣减等业务行为
// 业务不变式：stock_num >= 0
// ============================================

use chrono::{DateTime, Utc}; // 时间类型
use crate::domain::{DomainError, DomainResult}; // 领域错误

/// DDD 聚合根 - 库存领域实体
/// 维护库存数量不变式（非负），提供 increase/decrease 领域行为
#[derive(Clone, Debug)]
pub struct Stock {
    stock_id: i32,                        // 库存记录编号（数据库自增主键）
    material_id: String,                  // 物资编号（外键关联 Material）
    warehouse_id: String,                 // 仓库编号（外键关联 Warehouse）
    stock_num: i32,                       // 当前库存数量（私有，通过 increase/decrease 修改）
    in_time: Option<DateTime<Utc>>,       // 首次入库时间
    update_time: Option<DateTime<Utc>>,   // 最后更新时间（私有，通过领域行为设置）
}

impl Stock {
    /// 获取当前库存数量（只读访问）
    /// 返回库存数量，用于库存可用性检查
    pub fn stock_num(&self) -> i32 {
        self.stock_num                     // 当前库存
    }

    /// 获取最后更新时间（只读访问）
    pub fn update_time(&self) -> &Option<DateTime<Utc>> {
        &self.update_time                  // 更新时间
    }

    /// 获取库存记录编号（只读访问）
    pub fn stock_id(&self) -> i32 {
        self.stock_id                      // 自增主键
    }

    /// 获取物资编号（只读访问）
    pub fn material_id(&self) -> &str {
        &self.material_id                  // 物资编号
    }

    /// 获取仓库编号（只读访问）
    pub fn warehouse_id(&self) -> &str {
        &self.warehouse_id                 // 仓库编号
    }

    /// 获取首次入库时间（只读访问）
    pub fn in_time(&self) -> &Option<DateTime<Utc>> {
        &self.in_time                      // 首次入库时间
    }

    /// 从仓储重建聚合根（用于从数据库加载已有数据）
    /// 不走工厂方法，因为已有数据已经满足业务约束
    pub fn reconstitute(
        stock_id: i32,                     // 库存记录编号
        material_id: String,               // 物资编号
        warehouse_id: String,              // 仓库编号
        stock_num: i32,                    // 库存数量
        in_time: Option<DateTime<Utc>>,    // 首次入库时间
        update_time: Option<DateTime<Utc>>, // 最后更新时间
    ) -> Self {
        Self {
            stock_id,                      // 自增ID
            material_id,                   // 物资编号
            warehouse_id,                  // 仓库编号
            stock_num,                     // 库存数量
            in_time,                       // 首次入库时间
            update_time,                   // 最后更新时间
        }
    }

    /// 入库增加库存（领域行为）
    /// 校验：quantity > 0
    /// 不变式维护后：stock_num >= 0
    pub fn increase(&mut self, quantity: i32) -> DomainResult<()> {
        if quantity <= 0 {                 // 入库数量必须为正数
            return Err(DomainError::InvalidOperation("入库数量必须大于0".to_string()));
        }
        self.stock_num += quantity;        // 累加库存数量
        self.update_time = Some(Utc::now()); // 更新修改时间为当前 UTC
        Ok(())
    }

    /// 出库扣减库存（领域行为，含库存不足校验）
    /// 校验：quantity > 0 且 stock_num >= quantity
    /// 不变式维护后：stock_num >= 0
    pub fn decrease(&mut self, quantity: i32) -> DomainResult<()> {
        if quantity <= 0 {                 // 出库数量必须为正数
            return Err(DomainError::InvalidOperation("出库数量必须大于0".to_string()));
        }
        if self.stock_num < quantity {     // 校验库存是否充足
            return Err(DomainError::InsufficientStock(
                format!("库存不足：当前库存 {}，需要出库 {}", self.stock_num, quantity)
            ));
        }
        self.stock_num -= quantity;        // 扣减库存数量
        self.update_time = Some(Utc::now()); // 更新修改时间为当前 UTC
        Ok(())
    }

    /// 工厂方法：创建新库存记录
    /// 校验：初始库存 > 0
    /// stock_id 设为 0（由数据库自增生成）
    pub fn new(
        material_id: String,               // 物资编号
        warehouse_id: String,              // 仓库编号
        quantity: i32,                     // 初始库存数量
    ) -> DomainResult<Self> {
        if quantity <= 0 {                 // 初始库存必须为正数（首次入库至少 1 件）
            return Err(DomainError::InvalidOperation("初始库存必须大于0".to_string()));
        }
        Ok(Self {
            stock_id: 0,                   // 新建记录，ID 由数据库自增生成
            material_id,                   // 物资编号
            warehouse_id,                  // 仓库编号
            stock_num: quantity,           // 初始库存数量
            in_time: Some(Utc::now()),     // 首次入库时间 = 当前 UTC
            update_time: Some(Utc::now()), // 创建时更新时间 = 当前 UTC
        })
    }
}
