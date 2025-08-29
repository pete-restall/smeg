use crate::docs;
use crate::{ConstUsize, HasConstUsizeValue};

#[doc = docs::side_by_side_md!("HasMcuCoreId")]
pub trait HasMcuCoreId {
    type NumberOfMcuCores: HasConstUsizeValue;

    #[doc = docs::side_by_side_md!("HasMcuCoreId.mcu_core_id")]
    fn mcu_core_id(&self) -> usize;
}

#[doc = docs::side_by_side_md!("McuSingleCore")] // TODO: docs need updating
pub trait McuSingleCore: HasMcuCoreId { }

impl<T: McuSingleCore> HasMcuCoreId for T {
    type NumberOfMcuCores = ConstUsize<1>;

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
    fn NumberOfMcuCores__get_VALUE__expect_one() {
        expect!(<<StubSingleCoreMcu as HasMcuCoreId>::NumberOfMcuCores>::VALUE).to_equal(1);
    }

    #[test]
    fn mcu_core_id__called__expect_hard_coded_zero() {
        expect!(StubSingleCoreMcu.mcu_core_id()).to_equal(0);
    }
}
