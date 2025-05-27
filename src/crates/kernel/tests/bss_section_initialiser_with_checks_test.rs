#![allow(non_snake_case)]

use std::mem::MaybeUninit;

use fluent_test::prelude::*;

use smeg_kernel::errors::KernelErrorCode;
use smeg_kernel::bootstrapping::rust::{BssSectionInitialiser, BssSectionInitialiserWithChecks};

use smeg_testing_host_utils::integers::any_u8;
use smeg_testing_integration::despair::DespairMatchers;

#[test]
fn fill_bss_section__called_with_start_after_past_end__expect_despair() {
    _fill_bss_section__called_with_start_after_past_end__expect_despair::<2, 1>();
    _fill_bss_section__called_with_start_after_past_end__expect_despair::<3, 2>();
    _fill_bss_section__called_with_start_after_past_end__expect_despair::<10, 7>();
}

fn _fill_bss_section__called_with_start_after_past_end__expect_despair<const BUF_SIZE: usize, const INDEX: usize>() {
    expect!(|| unsafe {
        let mut buf = [MaybeUninit::<usize>::uninit(); BUF_SIZE];
        let (invalid_start, invalid_past_end) = invalid_ordering_split_at(INDEX, &mut buf);
        BssSectionInitialiserWithChecks.fill_bss_section(invalid_start, invalid_past_end, any_fill());
    }).to_despair_with_error_code(KernelErrorCode::LinkerScriptDespair);
}

fn invalid_ordering_split_at(index: usize, buf: &mut [MaybeUninit<usize>]) -> (&mut MaybeUninit<usize>, &MaybeUninit<usize>) {
    let (buf1, buf2) = buf.split_at_mut(index);
    let past_end_before_start = &buf1[0];
    let start_after_past_end = &mut buf2[0];
    (start_after_past_end, past_end_before_start)
}

fn any_fill() -> u8 {
    any_u8()
}
