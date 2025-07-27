#![allow(non_snake_case)]

use fluent_test::prelude::*;

use smeg_kernel::despair;
use smeg_kernel::errors::KernelErrorCode;
use smeg_kernel::errors::test_doubles::any_kernel_error_code;

use smeg_testing_integration::despair::DespairMatchers;

#[test]
fn despair__called_using_with_and_because__expect_despair_handler_is_called_with_same_error_code() {
    let error_code = any_kernel_error_code();
    expect!(|| { despair!(with(error_code), because("something", "bad", "happened")); })
        .to_despair_with_error_code(error_code);
}

#[test]
fn despair__called_using_with__expect_despair_handler_is_called_with_same_error_code() {
    let error_code = any_kernel_error_code();
    expect!(|| { despair!(with(error_code)); }).to_despair_with_error_code(error_code);
}

#[test]
fn despair__called_using_because__expect_despair_handler_is_called_with_error_code_for_general_despair() {
    expect!(|| { despair!(because("general despair is to be expected")); })
        .to_despair_with_error_code(KernelErrorCode::GeneralDespair(0));
}
