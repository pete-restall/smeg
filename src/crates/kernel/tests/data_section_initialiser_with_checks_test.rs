#![allow(non_snake_case)]

use std::mem::MaybeUninit;

use fluent_test::prelude::*;

use smeg_kernel::errors::KernelErrorCode;
use smeg_kernel::bootstrapping::rust::{DataSectionInitialisation, DataSectionInitialiserWithChecks};

use smeg_testing_integration::despair::DespairMatchers;

#[test]
fn load_data_section__called_with_start_after_past_end__expect_despair() {
    _load_data_section__called_with_start_after_past_end__expect_despair::<2, 1>();
    _load_data_section__called_with_start_after_past_end__expect_despair::<3, 2>();
    _load_data_section__called_with_start_after_past_end__expect_despair::<10, 7>();
}

fn _load_data_section__called_with_start_after_past_end__expect_despair<const DATA_SIZE: usize, const INDEX: usize>() {
    expect!(|| unsafe {
        let mut ram = [MaybeUninit::new(0_usize); DATA_SIZE];
        let rom = [0_usize; DATA_SIZE];
        let (invalid_ram_start, invalid_ram_past_end) = invalid_ordering_split_at(INDEX, &mut ram);
        DataSectionInitialiserWithChecks::load_data_section(invalid_ram_start, invalid_ram_past_end, &rom[0]);
    }).to_despair_with_error_code(KernelErrorCode::LinkerScriptDespair);
}

fn invalid_ordering_split_at(index: usize, ram: &mut [MaybeUninit<usize>]) -> (&mut MaybeUninit<usize>, &MaybeUninit<usize>) {
    let (head, tail) = ram.split_at_mut(index);
    let past_end_before_start = &head[0];
    let start_after_past_end = &mut tail[0];
    (start_after_past_end, past_end_before_start)
}
