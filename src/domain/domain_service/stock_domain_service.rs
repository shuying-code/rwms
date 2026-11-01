// ============================================
// 库存领域服务
// 封装入库增加/出库扣减的跨实体库存变动规则
// 纯领域逻辑，不依赖仓储，由 Application 层在事务中调用
// ============================================

use crate::domain::aggregate::stock::Stock;
use crate::domain::DomainResult;

/// 库存领域服务（无状态，所有方法为关联函数）
pub struct StockDomainService;

impl StockDomainService {
    /// 入库审核通过后增加库存
    ///
    /// # 参数
    /// * `existing_stock` - 已有库存（Some 则更新，None 则新建）
    /// * `material_id` - 物资编号
    /// * `warehouse_id` - 仓库编号
    /// * `quantity` - 入库数量
    ///
    /// # 返回
    /// * `Ok(Some(Stock))` - 需要新建库存记录
    /// * `Ok(None)` - 已更新现有库存
    pub fn increase_stock_on_inbound(
        existing_stock: Option<&mut Stock>,    // 已有库存引用，None 表示首次入库该物资
        material_id: &str,                     // 物资编号（仅新建库存时使用）
        warehouse_id: &str,                    // 仓库编号（仅新建库存时使用）
        quantity: i32,                          // 入库数量
    ) -> DomainResult<Option<Stock>> {
        match existing_stock {
            Some(stock) => {                    // 已有库存记录：直接增加数量
                stock.increase(quantity)?;      // 委托实体方法执行增加逻辑
                Ok(None)                        // 返回 None 表示已更新现有记录，无需新建
            }
            None => {                           // 无库存记录：需要新建
                let new_stock = Stock::new(
                    material_id.to_string(),    // 物资编号
                    warehouse_id.to_string(),   // 仓库编号
                    quantity,                   // 初始库存数量
                )?;
                Ok(Some(new_stock))             // 返回新建的库存实体，由应用层负责持久化
            }
        }
    }

    /// 出库审核通过后扣减库存
    pub fn decrease_stock_on_outbound(
        stock: &mut Stock,                      // 待扣减的库存实体（可变引用）
        quantity: i32,                           // 出库数量
    ) -> DomainResult<()> {
        stock.decrease(quantity)                // 委托实体方法执行扣减（含库存不足校验）
    }
}
