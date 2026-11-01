// ============================================
// 入库单领域实体（富领域模型）
// DDD 聚合根（Aggregate Root）
// 聚合边界：InOrder + InOrderItem
// 含状态转换、明细管理、审核行为
// ============================================

use chrono::{DateTime, Utc}; // 时间类型
use serde::ser::{Serialize, Serializer, SerializeStruct}; // 序列化
use super::order_status::OrderStatus; // 订单状态值对象
use super::order_type::OrderType; // 订单类型值对象
use super::in_order_item::InOrderItem; // 入库单明细实体
use super::contact::Contact; // 联系人值对象
use super::phone::Phone; // 电话号码值对象
use crate::domain::{DomainError, DomainResult}; // 领域错误

/// DDD 聚合根 - 入库单领域实体
/// 管理入库明细列表，控制状态转换（Pending→Approved/Rejected）
#[derive(Clone, Debug)]
pub struct InOrder {
    order_id: String,                     // 入库单号（业务主键）
    warehouse_id: String,                 // 目标仓库编号
    supplier_id: i32,                     // 供应商编号（外键关联 Supplier）
    operator_id: String,                  // 操作员编号（外键关联 SysUser）
    contact: Option<Contact>,             // 联系人（值对象，可选）
    phone: Option<Phone>,                 // 联系电话（值对象，可选）
    order_status: OrderStatus,            // 订单状态（私有，通过 approve/reject 修改）
    order_type: OrderType,                // 订单类型（值对象，固定为 Inbound）
    purchase_time: Option<DateTime<Utc>>, // 采购时间
    complete_time: Option<DateTime<Utc>>, // 审核完成时间（私有，通过 approve/reject 设置）
    remark: Option<String>,               // 备注（可选，自由文本）
    items: Vec<InOrderItem>,              // 聚合内部实体：入库单明细（由聚合根管理）
}

impl InOrder {
    /// 获取订单状态（只读访问）
    /// 用于应用层查询状态以决定分支逻辑
    pub fn order_status(&self) -> &OrderStatus {
        &self.order_status                 // 当前状态
    }

    /// 获取审核完成时间（只读访问）
    pub fn complete_time(&self) -> &Option<DateTime<Utc>> {
        &self.complete_time                // 审核完成时间
    }

    // === Getter 方法（只读访问，保持封装） ===

    /// 获取入库单号（只读）
    pub fn order_id(&self) -> &str {
        &self.order_id                     // 入库单号
    }

    /// 获取目标仓库编号（只读）
    pub fn warehouse_id(&self) -> &str {
        &self.warehouse_id                 // 仓库编号
    }

    /// 获取供应商编号（只读）
    pub fn supplier_id(&self) -> i32 {
        self.supplier_id                   // 供应商编号
    }

    /// 获取操作员编号（只读）
    pub fn operator_id(&self) -> &str {
        &self.operator_id                  // 操作员编号
    }

    /// 获取联系人（只读）
    pub fn contact(&self) -> &Option<Contact> {
        &self.contact                      // 联系人值对象
    }

    /// 获取联系电话（只读）
    pub fn phone(&self) -> &Option<Phone> {
        &self.phone                        // 电话值对象
    }

    /// 获取订单类型（只读）
    pub fn order_type(&self) -> &OrderType {
        &self.order_type                   // 订单类型值对象
    }

    /// 获取采购时间（只读）
    pub fn purchase_time(&self) -> &Option<DateTime<Utc>> {
        &self.purchase_time                // 采购时间
    }

    /// 获取备注（只读）
    pub fn remark(&self) -> &Option<String> {
        &self.remark                       // 备注
    }

    /// 工厂方法：创建新入库单
    /// 自动设置状态为 Pending，类型为 Inbound，complete_time 为 None
    pub fn new(
        order_id: String,                  // 入库单号
        warehouse_id: String,              // 目标仓库编号
        supplier_id: i32,                  // 供应商编号
        operator_id: String,               // 操作员编号
        contact: Option<Contact>,          // 联系人（值对象）
        phone: Option<Phone>,              // 联系电话（值对象）
        purchase_time: Option<DateTime<Utc>>, // 采购时间
        remark: Option<String>,            // 备注
    ) -> Self {
        Self {
            order_id,                      // 入库单号
            warehouse_id,                  // 仓库编号
            supplier_id,                   // 供应商编号
            operator_id,                   // 操作员编号
            contact,                       // 联系人
            phone,                         // 电话
            order_status: OrderStatus::Pending, // 初始状态：待审核
            order_type: OrderType::Inbound, // 固定类型：入库
            purchase_time,                 // 采购时间
            complete_time: None,           // 新建单无审核完成时间
            remark,                        // 备注
            items: Vec::new(),             // 空明细列表
        }
    }

    /// 向聚合内添加入库明细（聚合根负责管理子实体）
    /// 规则：只能在 Pending 状态下添加
    pub fn add_item(&mut self, mut item: InOrderItem) -> DomainResult<()> {
        if self.order_status != OrderStatus::Pending { // 状态守卫：只有待审核可添加
            return Err(DomainError::InvalidOperation("只能在待审核状态下添加明细".to_string()));
        }
        item.set_order_id(self.order_id.clone()); // 设置明细所属入库单号
        self.items.push(item);                    // 添加到聚合内明细列表
        Ok(())
    }

    /// 获取聚合内所有入库明细（只读访问）
    /// 返回引用，避免克隆
    pub fn get_items(&self) -> &Vec<InOrderItem> {
        &self.items                        // 明细列表只读引用
    }

    /// 设置聚合内的明细列表（从仓储加载时使用）
    /// 仅 aggregrate 模块和仓储实现可调用
    #[allow(dead_code)]
    pub(crate) fn set_items(&mut self, items: Vec<InOrderItem>) {
        self.items = items;                // 设置明细列表
    }

    /// 更新入库单信息（领域行为）
    /// 规则：仅 Pending 状态允许修改
    pub fn update(
        &mut self,
        warehouse_id: String,              // 新仓库编号
        supplier_id: i32,                  // 新供应商编号
        operator_id: String,               // 新操作员编号
        contact: Option<Contact>,          // 新联系人
        phone: Option<Phone>,              // 新联系电话
        purchase_time: Option<DateTime<Utc>>, // 新采购时间
        remark: Option<String>,            // 新备注
    ) -> DomainResult<()> {
        if self.order_status != OrderStatus::Pending { // 状态守卫：只有待审核可修改
            return Err(DomainError::InvalidOperation("只能在待审核状态下修改入库单".to_string()));
        }
        self.warehouse_id = warehouse_id;  // 更新仓库
        self.supplier_id = supplier_id;    // 更新供应商
        self.operator_id = operator_id;    // 更新操作员
        self.contact = contact;            // 更新联系人
        self.phone = phone;                // 更新电话
        self.purchase_time = purchase_time; // 更新采购时间
        self.remark = remark;              // 更新备注
        Ok(())
    }

    /// 从仓储重建聚合根（用于从数据库加载已有数据）
    /// 不走工厂方法，因为已有数据已经满足业务约束
    pub fn reconstitute(
        order_id: String,                  // 入库单号
        warehouse_id: String,              // 仓库编号
        supplier_id: i32,                  // 供应商编号
        operator_id: String,               // 操作员编号
        contact: Option<Contact>,          // 联系人
        phone: Option<Phone>,              // 电话
        order_status: OrderStatus,         // 当前状态
        order_type: OrderType,             // 订单类型
        purchase_time: Option<DateTime<Utc>>, // 采购时间
        complete_time: Option<DateTime<Utc>>, // 审核完成时间
        remark: Option<String>,            // 备注
        items: Vec<InOrderItem>,           // 明细列表
    ) -> Self {
        Self {
            order_id,                      // 入库单号
            warehouse_id,                  // 仓库编号
            supplier_id,                   // 供应商编号
            operator_id,                   // 操作员编号
            contact,                       // 联系人
            phone,                         // 电话
            order_status,                  // 状态
            order_type,                    // 类型
            purchase_time,                 // 采购时间
            complete_time,                 // 完成时间
            remark,                        // 备注
            items,                         // 明细列表
        }
    }

    /// 审核通过（领域行为：状态转换 + 设置完成时间）
    /// 规则：只能从 Pending 状态转换到 Approved
    pub fn approve(&mut self) -> DomainResult<()> {
        if self.order_status != OrderStatus::Pending { // 状态守卫：只能审核待审核的订单
            return Err(DomainError::InvalidOperation("只能审核待审核的订单".to_string()));
        }
        self.order_status = OrderStatus::Approved; // 状态 → 审核通过
        self.complete_time = Some(Utc::now());     // 记录审核完成时间
        Ok(())
    }

    /// 审核拒绝（领域行为：状态转换 + 设置完成时间）
    /// 规则：只能从 Pending 状态转换到 Rejected
    pub fn reject(&mut self) -> DomainResult<()> {
        if self.order_status != OrderStatus::Pending { // 状态守卫：只能审核待审核的订单
            return Err(DomainError::InvalidOperation("只能审核待审核的订单".to_string()));
        }
        self.order_status = OrderStatus::Rejected; // 状态 → 审核不通过
        self.complete_time = Some(Utc::now());     // 记录审核完成时间
        Ok(())
    }
}

impl Serialize for InOrder {
    /// JSON 序列化：领域实体 → API 响应
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("InOrder", 12)?; // 12 个字段
        s.serialize_field("order_id", &self.order_id)?;           // 入库单号
        s.serialize_field("warehouse_id", &self.warehouse_id)?;   // 仓库编号
        s.serialize_field("supplier_id", &self.supplier_id)?;     // 供应商编号
        s.serialize_field("operator_id", &self.operator_id)?;     // 操作员编号
        s.serialize_field("contact", &self.contact)?;             // 联系人
        s.serialize_field("phone", &self.phone)?;                 // 电话
        s.serialize_field("order_status", &self.order_status.as_str())?; // 订单状态（字符串）
        s.serialize_field("order_type", &self.order_type.as_str())?; // 订单类型（字符串）
        s.serialize_field("purchase_time", &self.purchase_time)?; // 采购时间
        s.serialize_field("complete_time", &self.complete_time)?; // 完成时间
        s.serialize_field("remark", &self.remark)?;               // 备注
        s.serialize_field("items", &self.items)?;                 // 明细列表
        s.end()
    }
}
