#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use crate::{ConstUsize, HasMcuCoreId};
use crate::bootstrapping::BootstrapperContext;
use crate::bootstrapping::kernel::McuCoreBootstrapping;

#[derive(Default)]
#[doc = docs::side_by_side_md!("StubForConstantMcuCoreId")]
pub struct StubForConstantMcuCoreId<const MCU_CORE_ID: usize>;

impl<const MCU_CORE_ID: usize> StubForConstantMcuCoreId<MCU_CORE_ID> {
    const _ENSURE_ID_LESS_THAN_LIMIT: () = assert!(MCU_CORE_ID < 32, "StubForConstantMcuCoreId must not have more than 32 cores");
}

impl<const MCU_CORE_ID: usize> BootstrapperContext for StubForConstantMcuCoreId<MCU_CORE_ID> { }

impl<const MCU_CORE_ID: usize> HasMcuCoreId for StubForConstantMcuCoreId<MCU_CORE_ID> {
    type NumberOfMcuCores = ConstUsize<32>;

    fn mcu_core_id(&self) -> usize { MCU_CORE_ID }
}

impl<const MCU_CORE_ID: usize> McuCoreBootstrapping for StubForConstantMcuCoreId<MCU_CORE_ID> {
    type McuCoreId = StubForConstantMcuCoreId<MCU_CORE_ID>;
}
