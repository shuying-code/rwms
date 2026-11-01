// ============================================
// 入库单领域服务
// 封装入库单创建校验、审核流程中的跨实体业务规则
// 纯领域逻辑，不依赖仓储，由 Application 层在事务中调用
// ============================================

use crate::domain::aggregate::in_order::InOrder;
use crate::domain::aggregate::in_order_item::InOrderItem;
use crate::domain::aggregate::stock::Stock;
use crate::domain::domain_service::stock_domain_service::StockDomainService;
use crate::domain::{DomainError, DomainResult};

/// 入库单领域服务（无状态，所有方法为关联函数）
pub struct InOrderDomainService;

/// 库存变更结果（审核通过后应用层据此持久化）
pub enum StockChange {
    /// 需要新建库存记录
    CreateNew(Stock),
    /// 需要更新现有库存记录
    UpdateExisting(Stock),
}

impl InOrderDomainService {
    /// 校验入库明细
    ///
    /// 规则：
    /// - 至少一条明细
    /// - 每条明细数量 > 0
    /// - 采购价 >= 0
    pub fn validate_in_order_items(items: &[InOrderItem]) -> DomainResult<()> {
        if items.is_empty() {                   // 校验：入库单至少包含一条明细
            return Err(DomainError::BusinessRuleViolation(
                "入库单至少需要一条明细".to_string(),
            ));
        }
        for item in items {                     // 逐条校验明细
            if item.num() <= 0 {                  // 校验：入库数量必须为正数
                return Err(DomainError::InvalidOperation(format!(
                    "入库数量必须大于0，物资：{}",
                    item.material_id()
                )));
            }
            if item.purchase_price() < rust_decimal::Decimal::ZERO {  // 校验：采购价不能为负数
                return Err(DomainError::InvalidOperation(format!(
                    "采购价不能为负数，物资：{}",
                    item.material_id()
                )));
            }
        }
        Ok(())
    }

    /// 入库审核通过核心逻辑
    ///
    /// 流程：
    /// 1. 调用 order.approve() 执行状态转换
    /// 2. 逐项调用 StockDomainService 增加库存
    /// 3. 返回每个明细对应的库存变更
    ///
    /// # 参数
    /// * `order` - 入库单实体（可变，会执行 approve 状态转换）
    /// * `items` - 入库明细列表
    /// * `existing_stocks` - 与 items 一一对应的已有库存（None 表示无库存）
    pub fn process_inbound_approval(
        order: &mut InOrder,                    // 入库单实体（可变引用，执行状态转换）
        items: &[InOrderItem],                  // 入库明细列表
        existing_stocks: Vec<Option<Stock>>,    // 每条明细对应的已有库存，与 items 一一对应
    ) -> DomainResult<Vec<StockChange>> {
        // 领域行为：审核通过（含状态守卫校验）
        order.approve()?;

        let mut changes = Vec::with_capacity(items.len());  // 预分配容量，避免频繁扩容
        for (item, existing) in items.iter().zip(existing_stocks.into_iter()) {  // 逐项处理：明细与库存一一配对
            let mut existing_mut = existing;                // 转为可变，供 StockDomainService 修改
            let result = StockDomainService::increase_stock_on_inbound(
                existing_mut.as_mut(),                      // 传入已有库存的可变引用（或 None）
                item.material_id(),                         // 物资编号
                &order.warehouse_id(),                       // 仓库编号
                item.num(),                                 // 入库数量
            )?;

            match result {
                Some(new_stock) => changes.push(StockChange::CreateNew(new_stock)),  // 返回新建的库存实体
                None => {                                   // 已更新现有库存
                    if let Some(updated) = existing_mut {   // 取出被修改过的库存实体
                        changes.push(StockChange::UpdateExisting(updated));
                    }
                }
            }
        }
        Ok(changes)                           // 返回所有库存变更，由应用层负责持久化
    }
}
