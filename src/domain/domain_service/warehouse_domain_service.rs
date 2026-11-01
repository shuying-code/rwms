// ============================================
// 仓库领域服务
// 封装仓库容量管理（入库占用/出库释放）的跨实体业务规则
// 纯领域逻辑，不依赖仓储，由 Application 层在事务中调用
// ============================================

use crate::domain::aggregate::warehouse::Warehouse;
use crate::domain::{DomainError, DomainResult};

/// 仓库领域服务（无状态，所有方法为关联函数）
#[allow(dead_code)]
pub struct WarehouseDomainService;

#[allow(dead_code)]
impl WarehouseDomainService {
    /// 校验仓库是否有足够剩余空间
    pub fn can_accommodate(warehouse: &Warehouse, additional_quantity: i32) -> DomainResult<()> {
        if additional_quantity <= 0 {           // 校验：占用数量必须为正数
            return Err(DomainError::InvalidOperation(
                "入库数量必须大于0".to_string(),
            ));
        }
        if warehouse.remain_space() < additional_quantity {  // 校验：剩余空间是否充足
            return Err(DomainError::BusinessRuleViolation(format!(
                "仓库 {} 剩余空间不足：剩余 {}，需要 {}",
                warehouse.warehouse_id(), warehouse.remain_space(), additional_quantity
            )));
        }
        Ok(())                                 // 空间充足，允许入库
    }

    /// 入库时占用仓库空间
    ///
    /// 校验容量充足后调用 Warehouse 实体的 allocate_space 方法
    pub fn allocate_space(warehouse: &mut Warehouse, quantity: i32) -> DomainResult<()> {
        Self::can_accommodate(warehouse, quantity)?;  // 先校验容量是否充足
        warehouse.allocate_space(quantity)?;          // 委托实体方法执行占用
        Ok(())
    }

    /// 出库时释放仓库空间
    ///
    /// 调用 Warehouse 实体的 release_space 方法
    pub fn release_space(warehouse: &mut Warehouse, quantity: i32) -> DomainResult<()> {
        if quantity <= 0 {                     // 校验：释放数量必须为正数
            return Err(DomainError::InvalidOperation(
                "释放数量必须大于0".to_string(),
            ));
        }
        warehouse.release_space(quantity)?;    // 委托实体方法执行释放
        Ok(())
    }
}
