use crate::HasMcuCoreId;
use crate::bootstrapping::BootstrapperContext;

pub trait McuCoreBootstrapping {
    type McuCoreId: BootstrapperContext + Default + HasMcuCoreId;

    fn mcu_core_id() -> usize { Self::McuCoreId::default().mcu_core_id() }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use super::super::test_doubles::mcu_core_bootstrapping::StubForConstantMcuCoreId;
    use super::*;

    #[test]
    fn mcu_core_id__called__expect_same_value_as_mcu_core_id_associated_type() {
        _mcu_core_id__called__expect_same_value_as_mcu_core_id_associated_type::<0>();
        _mcu_core_id__called__expect_same_value_as_mcu_core_id_associated_type::<1>();
        _mcu_core_id__called__expect_same_value_as_mcu_core_id_associated_type::<2>();
        _mcu_core_id__called__expect_same_value_as_mcu_core_id_associated_type::<89>();
    }

    fn _mcu_core_id__called__expect_same_value_as_mcu_core_id_associated_type<const MCU_CORE_ID: usize>() {
        struct Stub<const MCU_CORE_ID: usize>;
        impl<const MCU_CORE_ID: usize> McuCoreBootstrapping for Stub<MCU_CORE_ID> {
            type McuCoreId = StubForConstantMcuCoreId<MCU_CORE_ID>;
        }

        expect!(Stub::<MCU_CORE_ID>::mcu_core_id()).to_equal(MCU_CORE_ID);
    }
}
