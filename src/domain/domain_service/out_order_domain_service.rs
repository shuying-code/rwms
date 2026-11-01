// ============================================
// 出库单领域服务
// 封装出库单创建校验、审核流程中的跨实体业务规则
// 纯领域逻辑，不依赖仓储，由 Application 层在事务中调用
// ============================================

use crate::domain::aggregate::out_order::OutOrder;
use crate::domain::aggregate::out_order_item::OutOrderItem;
use crate::domain::aggregate::stock::Stock;
use crate::domain::domain_service::stock_domain_service::StockDomainService;
use crate::domain::{DomainError, DomainResult};

/// 出库单领域服务（无状态，所有方法为关联函数）
pub struct OutOrderDomainService;

/// 出库库存变更结果
pub struct OutboundStockChange {
    #[allow(dead_code)]
    pub material_id: String,                   // 物资编号（用于追踪和日志）
    pub stock: Stock,                          // 扣减后的库存实体，由应用层负责持久化
}

impl OutOrderDomainService {
    /// 校验出库明细
    ///
    /// 规则：
    /// - 至少一条明细
    /// - 每条明细数量 > 0
    /// - 销售价 >= 0
    pub fn validate_out_order_items(items: &[OutOrderItem]) -> DomainResult<()> {
        if items.is_empty() {                   // 校验：出库单至少包含一条明细
            return Err(DomainError::BusinessRuleViolation(
                "出库单至少需要一条明细".to_string(),
            ));
        }
        for item in items {                     // 逐条校验明细
            if item.num() <= 0 {                  // 校验：出库数量必须为正数
                return Err(DomainError::InvalidOperation(format!(
                    "出库数量必须大于0，物资：{}",
                    item.material_id()
                )));
            }
            if item.sell_price() < rust_decimal::Decimal::ZERO {  // 校验：销售价不能为负数
                return Err(DomainError::InvalidOperation(format!(
                    "销售价不能为负数，物资：{}",
                    item.material_id()
                )));
            }
        }
        Ok(())
    }

    /// 出库前库存可用性预检
    ///
    /// 逐项检查库存是否存在且数量充足，不修改任何状态
    pub fn check_outbound_stock_availability(
        items: &[OutOrderItem],                 // 出库明细列表
        stocks: &[Option<Stock>],               // 每条明细对应的已有库存，与 items 一一对应
    ) -> DomainResult<()> {
        for (item, stock_opt) in items.iter().zip(stocks.iter()) {  // 逐项检查：明细与库存一一配对
            match stock_opt {
                None => {                       // 无库存记录，直接拒绝
                    return Err(DomainError::InsufficientStock(format!(
                        "物资 {} 在该仓库无库存",
                        item.material_id()
                    )))
                }
                Some(stock) => {
                    if stock.stock_num() < item.num() {  // 库存数量不足
                        return Err(DomainError::InsufficientStock(format!(
                            "物资 {} 库存不足：当前库存 {}，需要出库 {}",
                            item.material_id(), stock.stock_num(), item.num()
                        )));
                    }
                }
            }
        }
        Ok(())                                 // 所有明细的库存均充足
    }

    /// 出库审核通过核心逻辑
    ///
    /// 流程：
    /// 1. 库存可用性预检
    /// 2. 调用 order.approve() 执行状态转换
    /// 3. 逐项调用 StockDomainService 扣减库存
    /// 4. 返回库存变更列表
    pub fn process_outbound_approval(
        order: &mut OutOrder,                   // 出库单实体（可变引用，执行状态转换）
        items: &[OutOrderItem],                 // 出库明细列表
        mut stocks: Vec<Option<Stock>>,         // 每条明细对应的已有库存（可变，执行扣减）
    ) -> DomainResult<Vec<OutboundStockChange>> {
        // 先做库存可用性预检（全部通过后才执行扣减，避免部分扣减）
        Self::check_outbound_stock_availability(items, &stocks)?;

        // 领域行为：审核通过（含状态守卫校验）
        order.approve()?;

        let mut changes = Vec::with_capacity(items.len());  // 预分配容量
        for (item, stock_opt) in items.iter().zip(stocks.iter_mut()) {  // 逐项扣减库存
            if let Some(stock) = stock_opt {    // 预检已确保库存存在，此处安全解包
                StockDomainService::decrease_stock_on_outbound(stock, item.num())?;  // 委托领域服务扣减
                changes.push(OutboundStockChange {
                    material_id: item.material_id().to_string(),  // 记录物资编号
                    stock: stock.clone(),                     // 记录扣减后的库存实体
                });
            }
        }
        Ok(changes)                           // 返回所有库存变更，由应用层负责持久化
    }
}
