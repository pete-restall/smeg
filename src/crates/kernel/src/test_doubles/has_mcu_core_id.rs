#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use crate::{ConstUsize, HasMcuCoreId};

use super::Dummy;

impl HasMcuCoreId for Dummy {
    #[doc = docs::side_by_side_md!("Dummy.NumberOfMcuCores")]
    type NumberOfMcuCores = ConstUsize<1>;

    #[doc = docs::side_by_side_md!("Dummy.mcu_core_id")]
    fn mcu_core_id(&self) -> usize { usize::default() }
}

#[doc = docs::side_by_side_md!("StubForConstantMcuCoreId")]
pub struct StubForConstantMcuCoreId<const MCU_CORE_ID: usize>;

impl<const MCU_CORE_ID: usize> StubForConstantMcuCoreId<MCU_CORE_ID> {
    const _ENSURE_ID_LESS_THAN_LIMIT: () = assert!(MCU_CORE_ID < 32, "StubForConstantMcuCoreId must not have more than 32 cores");
}

impl<const MCU_CORE_ID: usize> HasMcuCoreId for StubForConstantMcuCoreId<MCU_CORE_ID> {
    #[doc = docs::side_by_side_md!("StubForConstantMcuCoreId.NumberOfMcuCores")]
    type NumberOfMcuCores = ConstUsize<32>;

    #[doc = docs::side_by_side_md!("StubForConstantMcuCoreId.mcu_core_id")]
    fn mcu_core_id(&self) -> usize { MCU_CORE_ID }
}
