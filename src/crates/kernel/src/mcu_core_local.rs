use crate::docs;
use crate::{despair, ConstUsize, HasMcuCoreId};
use crate::errors::KernelErrorCode;

#[doc = docs::side_by_side_md!("McuCoreLocal")]
pub struct McuCoreLocal<'mcu, M, const N: usize, T> where
    M: 'mcu + HasMcuCoreId<NumberOfMcuCores = ConstUsize<N>>,
    [(); N]: Sized,
    T: 'mcu {

    mcu: &'mcu M,
    values: [T; N]
}

impl<'mcu, M, const N: usize, T> McuCoreLocal<'mcu, M, N, T> where
    M: 'mcu + HasMcuCoreId<NumberOfMcuCores = ConstUsize<N>>,
    [(); N]: Sized,
    T: 'mcu {

    #[doc = docs::side_by_side_md!("McuCoreLocal.new_all")]
    pub fn new_all(mcu: &'mcu M, value: T) -> Self where T: Copy {
        Self::new(mcu, [value; N])
    }

    #[doc = docs::side_by_side_md!("McuCoreLocal.new")]
    pub const fn new(mcu: &'mcu M, values: [T; N]) -> Self {
        Self { mcu, values }
    }

    #[doc = docs::side_by_side_md!("McuCoreLocal.with")]
    pub fn with<F, R>(&self, f: F) -> R where F: FnOnce(&T) -> R {
        let core_id = self.mcu.mcu_core_id();
        if core_id >= N {
            despair!(
                with(KernelErrorCode::InvalidMcuCoreId),
                because("MCU core IDs must be in the range of [0, N), where N is the number of cores"));
        }

        f(&self.values[core_id])
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use core::fmt::Debug;

    use fluent_test::prelude::*;

    use smeg_testing_host_utils::array::array_filled_using;
    use smeg_testing_host_utils::integers::{any_isize, any_usize, any_u8};

    use crate::HasConstUsizeValue;
    use crate::test_doubles::Dummy;
    use crate::test_doubles::has_mcu_core_id::{StubForConstantMcuCoreId, StubHasMcuCoreId};

    use super::*;

    #[test]
    fn mcu__get_after_new__expect_same_reference_passed_to_constructor() {
        let mcu = Dummy;
        let core_local = McuCoreLocal::new(&mcu, [0; <Dummy as HasMcuCoreId>::NumberOfMcuCores::VALUE]);
        expect!(core_local.mcu).to_equal(&mcu);
    }

    #[test]
    fn mcu__get_after_new_all__expect_same_reference_passed_to_constructor() {
        let mcu = Dummy;
        let core_local = McuCoreLocal::new_all(&mcu, Dummy);
        expect!(core_local.mcu).to_equal(&mcu);
    }

    #[test]
    fn values__get_after_new__expect_same_values_passed_to_constructor() {
        values__get_after_new_for_given_size__expect_same_values_passed_to_constructor::<1>();
        values__get_after_new_for_given_size__expect_same_values_passed_to_constructor::<2>();
        values__get_after_new_for_given_size__expect_same_values_passed_to_constructor::<13>();
        values__get_after_new_for_given_size__expect_same_values_passed_to_constructor::<{ usize::BITS as usize }>();
    }

    fn values__get_after_new_for_given_size__expect_same_values_passed_to_constructor<const N: usize>() {
        let mcu = StubForConstantMcuCoreId::<0, N>;
        let initial_values: [_; N] = array_filled_using(any_usize);
        let values = initial_values.clone();
        let core_local = McuCoreLocal::new(&mcu, values);
        expect!(core_local.values).to_equal(initial_values);
    }

    #[test]
    fn values__get_after_new_all__expect_repeated_value_passed_to_constructor() {
        values__get_after_new_all_for_given_size__expect_repeated_value_passed_to_constructor::<1, _>(any_usize());
        values__get_after_new_all_for_given_size__expect_repeated_value_passed_to_constructor::<2, _>(any_u8());
        values__get_after_new_all_for_given_size__expect_repeated_value_passed_to_constructor::<13, _>(any_usize());
        values__get_after_new_all_for_given_size__expect_repeated_value_passed_to_constructor::<{ usize::BITS as usize }, _>(any_isize());
    }

    fn values__get_after_new_all_for_given_size__expect_repeated_value_passed_to_constructor<const N: usize, T: Copy + Debug + PartialEq>(value: T) {
        let mcu = StubForConstantMcuCoreId::<0, N>;
        let core_local = McuCoreLocal::new_all(&mcu, value);
        expect!(core_local.values).to_equal([value; N]);
    }

    #[test]
    fn with__called__expect_closure_is_called_with_corresponding_shared_reference_to_mcu_core_local() {
        fn assert_for_number_of_cores<const N: usize>() {
            for core_id in 0..N {
                _with__called__expect_closure_is_called_with_corresponding_shared_reference_to_mcu_core_local::<N>(core_id);
            }
        }

        assert_for_number_of_cores::<1>();
        assert_for_number_of_cores::<3>();
        assert_for_number_of_cores::<10>();
    }

    fn _with__called__expect_closure_is_called_with_corresponding_shared_reference_to_mcu_core_local<const N: usize>(core_id: usize) {
        #[derive(Copy, Clone)]
        #[repr(C, packed)]
        struct PerCore { x: [u8; 5] }

        let mcu = StubHasMcuCoreId::<N>::with(core_id);
        let core_local = McuCoreLocal::new_all(&mcu, PerCore { x: [0; 5] });
        let core_local_ref = core_local.with(|x| &raw const *x);
        expect!(core_local_ref).to_equal(&raw const *core_local.values.get(core_id).unwrap());
    }
}
