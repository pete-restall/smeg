use crate::HasMcuCoreId;

pub trait McuCoreBootstrapping {
    type McuCoreId: HasMcuCoreId;

    fn core_id() -> usize { Self::McuCoreId::core_id() }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use crate::test_doubles::has_mcu_core_id::StubForConstantMcuCoreId;

    use super::*;

    #[test]
    fn core_id__called__expect_same_value_as_mcu_core_id_associated_type() {
        _core_id__called__expect_same_value_as_mcu_core_id_associated_type::<0>();
        _core_id__called__expect_same_value_as_mcu_core_id_associated_type::<1>();
        _core_id__called__expect_same_value_as_mcu_core_id_associated_type::<2>();
        _core_id__called__expect_same_value_as_mcu_core_id_associated_type::<89>();
    }

    fn _core_id__called__expect_same_value_as_mcu_core_id_associated_type<const MCU_CORE_ID: usize>() {
        struct Stub<const MCU_CORE_ID: usize>;
        impl<const MCU_CORE_ID: usize> McuCoreBootstrapping for Stub<MCU_CORE_ID> {
            type McuCoreId = StubForConstantMcuCoreId<MCU_CORE_ID>;
        }

        expect!(Stub::<MCU_CORE_ID>::core_id()).to_equal(MCU_CORE_ID);
    }
}
