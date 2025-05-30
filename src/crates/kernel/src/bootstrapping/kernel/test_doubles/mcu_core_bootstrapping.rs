#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use crate::bootstrapping::kernel::McuCoreBootstrapping;

#[doc = docs::side_by_side_md!("StubForConstantMcuCoreId")]
pub struct StubForConstantMcuCoreId<const MCU_CORE_ID: usize>;

impl<const MCU_CORE_ID: usize> McuCoreBootstrapping for StubForConstantMcuCoreId<MCU_CORE_ID> {
    type McuCoreId = crate::test_doubles::has_mcu_core_id::StubForConstantMcuCoreId<MCU_CORE_ID>;
}
