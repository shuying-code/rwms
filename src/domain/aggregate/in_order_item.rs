// ============================================
// 入库单明细领域实体
// DDD 实体（Entity），属于 InOrder 聚合
// 包含明细金额计算行为
// ============================================

use rust_decimal::Decimal; // 高精度数值类型
use serde::ser::{Serialize, Serializer, SerializeStruct}; // 序列化
use crate::domain::{DomainError, DomainResult}; // 领域错误

/// DDD 实体 - 入库单明细（属于 InOrder 聚合）
/// 由聚合根管理生命周期，不能独立存在
#[derive(Clone, Debug)]
pub struct InOrderItem {
    item_id: i32,                    // 明细编号（数据库自增主键）
    order_id: String,                // 所属入库单号（由聚合根设置）
    material_id: String,             // 物资编号（外键关联 Material）
    num: i32,                        // 入库数量
    purchase_price: Decimal,         // 采购单价（精度数值）
    sell_price: Decimal,             // 销售单价（精度数值，入库时记录当时售价）
    total_amount: Option<Decimal>,   // 明细总金额（= num × purchase_price，工厂方法自动计算）
}

impl InOrderItem {
    /// 工厂方法：创建入库明细
    /// 自动计算 total_amount = num × purchase_price
    /// 校验：数量 > 0，价格 >= 0
    pub fn new(
        material_id: String,      // 物资编号
        num: i32,                 // 入库数量
        purchase_price: Decimal,  // 采购单价
        sell_price: Decimal,      // 销售单价
    ) -> DomainResult<Self> {
        if num <= 0 {             // 入库数量必须为正数
            return Err(DomainError::InvalidOperation("入库数量必须大于0".to_string()));
        }
        if purchase_price < Decimal::ZERO { // 采购价不能为负数
            return Err(DomainError::InvalidOperation("采购价不能为负数".to_string()));
        }
        if sell_price < Decimal::ZERO {     // 销售价不能为负数
            return Err(DomainError::InvalidOperation("销售价不能为负数".to_string()));
        }
        let total_amount = Some(Decimal::from(num) * purchase_price); // 自动计算总金额
        Ok(Self {
            item_id: 0,           // 新记录 ID 由数据库生成
            order_id: String::new(), // 空字符串，由聚合根 add_item 时设置
            material_id,          // 物资编号
            num,                  // 入库数量
            purchase_price,       // 采购单价
            sell_price,           // 销售单价
            total_amount,         // 计算后的总金额
        })
    }

    /// 从仓储重建明细（已有数据，不做额外校验）
    pub fn reconstitute(
        item_id: i32,              // 明细编号
        order_id: String,          // 所属入库单号
        material_id: String,       // 物资编号
        num: i32,                  // 入库数量
        purchase_price: Decimal,   // 采购单价
        sell_price: Decimal,       // 销售单价
        total_amount: Option<Decimal>, // 总金额
    ) -> Self {
        Self {
            item_id,               // 已有 ID
            order_id,              // 入库单号
            material_id,           // 物资编号
            num,                   // 数量
            purchase_price,        // 采购价
            sell_price,            // 销售价
            total_amount,          // 总金额
        }
    }

    /// 设置所属入库单号（由聚合根 InOrder::add_item 调用）
    /// pub(super) 限制只在 aggregate 模块内可见
    pub(super) fn set_order_id(&mut self, order_id: String) {
        self.order_id = order_id;    // 设置为所属入库单号
    }

    /// 重新计算明细总金额
    /// 在修改数量和单价后调用以保持数据一致性
    pub fn calculate_total(&mut self) {
        self.total_amount = Some(Decimal::from(self.num) * self.purchase_price); // 总金额 = 数量 × 采购价
    }

    // === Getter 方法（只读访问，保持封装） ===

    /// 获取明细编号（只读）
    pub fn item_id(&self) -> i32 {
        self.item_id                 // 自增主键
    }

    /// 获取所属入库单号（只读）
    pub fn order_id(&self) -> &str {
        &self.order_id               // 入库单号
    }

    /// 获取物资编号（只读）
    pub fn material_id(&self) -> &str {
        &self.material_id            // 物资编号
    }

    /// 获取入库数量（只读）
    pub fn num(&self) -> i32 {
        self.num                     // 数量
    }

    /// 获取采购单价（只读）
    pub fn purchase_price(&self) -> Decimal {
        self.purchase_price          // 采购价（Decimal 实现 Copy）
    }

    /// 获取销售单价（只读）
    pub fn sell_price(&self) -> Decimal {
        self.sell_price              // 销售价（Decimal 实现 Copy）
    }

    /// 获取明细总金额（只读）
    pub fn total_amount(&self) -> Option<Decimal> {
        self.total_amount            // 总金额
    }
}

impl Serialize for InOrderItem {
    /// JSON 序列化：实体 → API 响应
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("InOrderItem", 7)?; // 7 个字段
        s.serialize_field("item_id", &self.item_id)?;               // 明细编号
        s.serialize_field("order_id", &self.order_id)?;             // 入库单号
        s.serialize_field("material_id", &self.material_id)?;       // 物资编号
        s.serialize_field("num", &self.num)?;                       // 数量
        s.serialize_field("purchase_price", &self.purchase_price)?; // 采购价
        s.serialize_field("sell_price", &self.sell_price)?;         // 销售价
        s.serialize_field("total_amount", &self.total_amount)?;     // 总金额
        s.end()
    }
}
