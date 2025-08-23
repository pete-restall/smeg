use crate::docs;

use core::num::NonZero;

#[doc = docs::side_by_side_md!("HasMcuCoreId")]
pub trait HasMcuCoreId {
    const NUMBER_OF_MCU_CORES: NonZero<usize>;

    #[doc = docs::side_by_side_md!("HasMcuCoreId.mcu_core_id")]
    fn mcu_core_id(&self) -> usize;
}

#[doc = docs::side_by_side_md!("McuSingleCore")] // TODO: docs need updating
pub trait McuSingleCore: HasMcuCoreId { }

impl<T: McuSingleCore> HasMcuCoreId for T {
    const NUMBER_OF_MCU_CORES: NonZero<usize> = NonZero::new(1).unwrap();

    #[doc = docs::side_by_side_md!("McuSingleCore.mcu_core_id")]
    fn mcu_core_id(&self) -> usize { 0 }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use super::*;

    struct StubSingleCoreMcu;
    impl McuSingleCore for StubSingleCoreMcu { }

    #[test]
    fn NUMBER_OF_MCU_CORES__get__expect_one() {
        expect!(StubSingleCoreMcu::NUMBER_OF_MCU_CORES.get()).to_equal(1);
    }

    #[test]
    fn mcu_core_id__called__expect_hard_coded_zero() {
        expect!(StubSingleCoreMcu.mcu_core_id()).to_equal(0);
    }
}
