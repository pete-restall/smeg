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
pub struct StubForConstantMcuCoreId<const MCU_CORE_ID: usize, const NUMBER_OF_MCU_CORES: usize = { usize::BITS as usize }>;

impl<const MCU_CORE_ID: usize, const NUMBER_OF_MCU_CORES: usize> StubForConstantMcuCoreId<MCU_CORE_ID, NUMBER_OF_MCU_CORES> {
    const _ENSURE_NUM_LESS_THAN_LIMIT: () =
        assert!(NUMBER_OF_MCU_CORES <= (usize::BITS as usize), "StubForConstantMcuCoreId must not have more cores than bits in a machine word");

    const _ENSURE_ID_LESS_THAN_NUM: () =
        assert!(MCU_CORE_ID < NUMBER_OF_MCU_CORES, "StubForConstantMcuCoreId must not have an ID >= the given number of cores");
}

impl<const MCU_CORE_ID: usize, const NUMBER_OF_MCU_CORES: usize> HasMcuCoreId for StubForConstantMcuCoreId<MCU_CORE_ID, NUMBER_OF_MCU_CORES> {
    #[doc = docs::side_by_side_md!("StubForConstantMcuCoreId.NumberOfMcuCores")]
    type NumberOfMcuCores = ConstUsize<NUMBER_OF_MCU_CORES>;

    #[doc = docs::side_by_side_md!("StubForConstantMcuCoreId.mcu_core_id")]
    fn mcu_core_id(&self) -> usize { MCU_CORE_ID }
}

#[doc = docs::side_by_side_md!("StubHasMcuCoreId")]
pub struct StubHasMcuCoreId<const NUMBER_OF_MCU_CORES: usize> {
    core_id: usize
}

impl<const NUMBER_OF_MCU_CORES: usize> StubHasMcuCoreId<NUMBER_OF_MCU_CORES> {
    const _ENSURE_NUM_LESS_THAN_LIMIT: () =
        assert!(NUMBER_OF_MCU_CORES <= (usize::BITS as usize), "StubHasMcuCoreId must not have more cores than bits in a machine word");

    #[doc = docs::side_by_side_md!("StubHasMcuCoreId.with")]
    pub fn with(core_id: usize) -> Self {
        assert!(core_id < NUMBER_OF_MCU_CORES, "StubHasMcuCoreId must not have an ID >= the given number of cores");
        Self { core_id }
    }

    #[doc = docs::side_by_side_md!("StubHasMcuCoreId.with_unchecked")]
    pub fn with_unchecked(core_id: usize) -> Self {
        Self { core_id }
    }
}

impl<const NUMBER_OF_MCU_CORES: usize> HasMcuCoreId for StubHasMcuCoreId<NUMBER_OF_MCU_CORES> {
    #[doc = docs::side_by_side_md!("StubHasMcuCoreId.NumberOfMcuCores")]
    type NumberOfMcuCores = ConstUsize<NUMBER_OF_MCU_CORES>;

    #[doc = docs::side_by_side_md!("StubHasMcuCoreId.mcu_core_id")]
    fn mcu_core_id(&self) -> usize { self.core_id }
}
