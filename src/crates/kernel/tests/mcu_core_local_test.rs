#![allow(non_snake_case)]

use fluent_test::prelude::*;

use smeg_kernel::McuCoreLocal;
use smeg_kernel::errors::KernelErrorCode;
use smeg_kernel::test_doubles::has_mcu_core_id::StubHasMcuCoreId;

use smeg_testing_integration::despair::DespairMatchers;

#[test]
fn with__called_when_core_id_is_out_of_bounds__expect_despair() {
    _with__called_when_core_id_is_out_of_bounds__expect_despair::<1>(1);
    _with__called_when_core_id_is_out_of_bounds__expect_despair::<1>(2);
    _with__called_when_core_id_is_out_of_bounds__expect_despair::<1>(15);
    _with__called_when_core_id_is_out_of_bounds__expect_despair::<5>(5);
    _with__called_when_core_id_is_out_of_bounds__expect_despair::<5>(6);
    _with__called_when_core_id_is_out_of_bounds__expect_despair::<5>(32);
}

fn _with__called_when_core_id_is_out_of_bounds__expect_despair<const N: usize>(bad_core_id: usize) {
    let mcu = StubHasMcuCoreId::<N>::with_unchecked(bad_core_id);
    let core_local = McuCoreLocal::new_all(&mcu, 0);
    expect!(|| {
        core_local.with(|_| panic!("Closure should not be called; expected an earlier panic"));
    }).to_despair_with_error_code(KernelErrorCode::InvalidMcuCoreId);
}
