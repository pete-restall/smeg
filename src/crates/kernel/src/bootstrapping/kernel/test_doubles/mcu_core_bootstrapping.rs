#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use core::num::NonZero;

use crate::HasMcuCoreId;
use crate::bootstrapping::BootstrapperContext;
use crate::bootstrapping::kernel::McuCoreBootstrapping;

#[derive(Default)]
#[doc = docs::side_by_side_md!("StubForConstantMcuCoreId")]
pub struct StubForConstantMcuCoreId<const MCU_CORE_ID: usize>;

impl<const MCU_CORE_ID: usize> BootstrapperContext for StubForConstantMcuCoreId<MCU_CORE_ID> { }

impl<const MCU_CORE_ID: usize> HasMcuCoreId for StubForConstantMcuCoreId<MCU_CORE_ID> {
    const NUMBER_OF_MCU_CORES: NonZero<usize> = NonZero::new(MCU_CORE_ID + 1).unwrap();

    fn mcu_core_id(&self) -> usize { MCU_CORE_ID }
}

impl<const MCU_CORE_ID: usize> McuCoreBootstrapping for StubForConstantMcuCoreId<MCU_CORE_ID> {
    type McuCoreId = StubForConstantMcuCoreId<MCU_CORE_ID>;
}
