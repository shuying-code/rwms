// ============================================
// 物资领域实体（富领域模型）
// DDD 聚合根（Aggregate Root）
// 包含价格非负验证
// ============================================

use chrono::{DateTime, Utc}; // 时间类型
use rust_decimal::Decimal; // 高精度数值（避免浮点误差）
use serde::ser::{Serialize, Serializer, SerializeStruct}; // 序列化
use crate::domain::{DomainError, DomainResult}; // 领域错误
use super::unit::Unit; // 计量单位值对象

/// DDD 聚合根 - 物资领域实体
#[derive(Clone, Debug)]
pub struct Material {
    material_id: String,              // 物资编号（业务主键）
    material_name: String,            // 物资名称
    category_id: i32,                 // 所属分类编号（外键关联 MaterialCategory）
    unit: Unit,                       // 计量单位（值对象，如"个"、"箱"）
    purchase_price: Decimal,          // 采购价（精度数值，非负）
    sell_price: Decimal,              // 销售价（精度数值，非负）
    total_stock: Option<i32>,         // 物资总库存（聚合值，可由库存表计算）
    create_time: Option<DateTime<Utc>>, // 创建时间
    update_time: Option<DateTime<Utc>>, // 最后更新时间
}

impl Material {
    /// 工厂方法：创建新物资
    /// 自动设置 total_stock = 0，create_time/update_time = 当前时间
    pub fn new(
        material_id: String,     // 物资编号（业务主键）
        material_name: String,   // 物资名称
        category_id: i32,        // 分类编号
        unit: Unit,              // 计量单位（值对象）
        purchase_price: Decimal, // 采购价
        sell_price: Decimal,     // 销售价
    ) -> Self {
        Self {
            material_id,         // 物资编号
            material_name,       // 物资名称
            category_id,         // 分类编号
            unit,                // 计量单位
            purchase_price,      // 采购价
            sell_price,          // 销售价
            total_stock: Some(0), // 新物资库存初始为 0
            create_time: Some(Utc::now()), // 创建时间
            update_time: Some(Utc::now()), // 更新时间
        }
    }

    /// 从仓储重建聚合根（用于从数据库加载已有数据）
    /// 不走工厂方法，因为已有数据已经满足业务约束
    pub fn reconstitute(
        material_id: String,              // 物资编号
        material_name: String,            // 物资名称
        category_id: i32,                 // 分类编号
        unit: Unit,                       // 计量单位
        purchase_price: Decimal,          // 采购价
        sell_price: Decimal,              // 销售价
        total_stock: Option<i32>,         // 总库存
        create_time: Option<DateTime<Utc>>, // 创建时间
        update_time: Option<DateTime<Utc>>, // 更新时间
    ) -> Self {
        Self {
            material_id,                 // 物资编号
            material_name,               // 物资名称
            category_id,                 // 分类编号
            unit,                        // 计量单位
            purchase_price,              // 采购价
            sell_price,                  // 销售价
            total_stock,                 // 总库存
            create_time,                 // 创建时间
            update_time,                 // 更新时间
        }
    }

    /// 更新物资信息（领域行为）
    /// 校验：名称非空、单位非空、价格非负
    pub fn update(
        &mut self,
        material_name: String,   // 新物资名称
        category_id: i32,        // 新分类编号
        unit: Unit,              // 新计量单位（值对象）
        purchase_price: Decimal, // 新采购价
        sell_price: Decimal,     // 新销售价
    ) -> DomainResult<()> {
        if material_name.trim().is_empty() {  // 物资名称不能为空
            return Err(DomainError::InvalidOperation("物资名称不能为空".to_string()));
        }
        if purchase_price < Decimal::ZERO {   // 采购价不能为负数
            return Err(DomainError::InvalidOperation("采购价不能为负数".to_string()));
        }
        if sell_price < Decimal::ZERO {       // 销售价不能为负数
            return Err(DomainError::InvalidOperation("销售价不能为负数".to_string()));
        }
        self.material_name = material_name;  // 更新名称
        self.category_id = category_id;      // 更新分类
        self.unit = unit;                    // 更新单位
        self.purchase_price = purchase_price; // 更新采购价
        self.sell_price = sell_price;        // 更新销售价
        self.update_time = Some(Utc::now()); // 更新修改时间
        Ok(())
    }

    // === Getter 方法（只读访问，保持封装） ===

    /// 获取物资编号（只读）
    pub fn material_id(&self) -> &str {
        &self.material_id             // 物资编号
    }

    /// 获取物资名称（只读）
    pub fn material_name(&self) -> &str {
        &self.material_name           // 物资名称
    }

    /// 获取分类编号（只读）
    pub fn category_id(&self) -> i32 {
        self.category_id              // 分类编号
    }

    /// 获取计量单位（只读）
    pub fn unit(&self) -> &Unit {
        &self.unit                    // 计量单位值对象
    }

    /// 获取采购价（只读）
    pub fn purchase_price(&self) -> Decimal {
        self.purchase_price           // 采购价（Decimal 实现了 Copy）
    }

    /// 获取销售价（只读）
    pub fn sell_price(&self) -> Decimal {
        self.sell_price               // 销售价（Decimal 实现了 Copy）
    }

    /// 获取总库存（只读）
    pub fn total_stock(&self) -> Option<i32> {
        self.total_stock              // 总库存（可能为空）
    }

    /// 获取创建时间（只读）
    pub fn create_time(&self) -> &Option<DateTime<Utc>> {
        &self.create_time             // 创建时间
    }

    /// 获取更新时间（只读）
    pub fn update_time(&self) -> &Option<DateTime<Utc>> {
        &self.update_time             // 更新时间
    }
}

impl Serialize for Material {
    /// JSON 序列化：领域实体 → API 响应
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Material", 9)?; // 9 个字段
        s.serialize_field("material_id", &self.material_id)?;     // 物资编号
        s.serialize_field("material_name", &self.material_name)?; // 物资名称
        s.serialize_field("category_id", &self.category_id)?;     // 分类编号
        s.serialize_field("unit", &self.unit.as_str())?;          // 计量单位（字符串）
        s.serialize_field("purchase_price", &self.purchase_price)?; // 采购价
        s.serialize_field("sell_price", &self.sell_price)?;       // 销售价
        s.serialize_field("total_stock", &self.total_stock)?;     // 总库存
        s.serialize_field("create_time", &self.create_time)?;     // 创建时间
        s.serialize_field("update_time", &self.update_time)?;     // 更新时间
        s.end()
    }
}
