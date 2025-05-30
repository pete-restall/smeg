#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use crate::HasMcuCoreId;

use super::Dummy;

impl HasMcuCoreId for Dummy {
    #[doc = docs::side_by_side_md!("Dummy.core_id")]
    fn core_id() -> usize { usize::default() }
}

#[doc = docs::side_by_side_md!("StubForConstantMcuCoreId")]
pub struct StubForConstantMcuCoreId<const MCU_CORE_ID: usize>;

impl<const MCU_CORE_ID: usize> HasMcuCoreId for StubForConstantMcuCoreId<MCU_CORE_ID> {
    #[doc = docs::side_by_side_md!("StubForConstantMcuCoreId.core_id")]
    fn core_id() -> usize { MCU_CORE_ID }
}
