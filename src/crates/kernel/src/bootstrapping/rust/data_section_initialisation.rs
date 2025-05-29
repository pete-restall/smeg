#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use core::mem::MaybeUninit;

use crate::despair;
use crate::errors::KernelErrorCode;

#[doc = docs::side_by_side_md!("DataSectionInitialisation")]
pub unsafe trait DataSectionInitialisation {
    #[doc = docs::side_by_side_md!("DataSectionInitialisation.load_data_section")]
    unsafe fn load_data_section(ram_start: &mut MaybeUninit<usize>, ram_past_end: &MaybeUninit<usize>, rom_start: &usize);
}

#[doc = docs::side_by_side_md!("DataSectionInitialiserWithChecks")]
pub struct DataSectionInitialiserWithChecks;

unsafe impl DataSectionInitialisation for DataSectionInitialiserWithChecks {
    #[doc = docs::side_by_side_md!("DataSectionInitialiserWithChecks.load_data_section")]
    unsafe fn load_data_section(ram_start: &mut MaybeUninit<usize>, ram_past_end: &MaybeUninit<usize>, rom_start: &usize) {
        unsafe {
            if ram_start.as_ptr() > ram_past_end.as_ptr() {
                despair!(with(KernelErrorCode::LinkerScriptDespair), because("Linker-supplied section pointers for .data are corrupt"));
            }

            DataSectionInitialiserWithoutChecks::load_data_section(ram_start, ram_past_end, rom_start)
        }
    }
}

#[doc = docs::side_by_side_md!("DataSectionInitialiserWithoutChecks")]
pub struct DataSectionInitialiserWithoutChecks;

unsafe impl DataSectionInitialisation for DataSectionInitialiserWithoutChecks {
    #[doc = docs::side_by_side_md!("DataSectionInitialiserWithoutChecks.load_data_section")]
    unsafe fn load_data_section(ram_start: &mut MaybeUninit<usize>, ram_past_end: &MaybeUninit<usize>, rom_start: &usize) {
        unsafe {
            let data_size_words = ram_past_end.as_ptr().offset_from(ram_start.as_ptr());
            core::ptr::copy_nonoverlapping(rom_start as *const usize, ram_start.as_mut_ptr(), data_size_words as usize);
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use core::mem::MaybeUninit;
    use core::ops::RangeInclusive;

    use fluent_test::prelude::*;

    use crate::bootstrapping::rust::DataSectionInitialisation;

    use smeg_testing_host_utils::integers::{any_usize, any_usize_except};
    use smeg_testing_host_utils::seq::EqualIteratorsMatchers;
    use smeg_testing_host_utils::vec::any_vec_filled_using;

    fn load_data_section__called_with_ram_start_equal_to_ram_past_end__expect_ram_is_not_modified<I: DataSectionInitialisation>() {
        let original_value = any_usize();
        let mut ram = [MaybeUninit::new(original_value); 1];
        let rom = [any_usize_except(original_value); 1];
        unsafe {
            let (ram_start, ram_past_end) = (&raw mut ram[0], &ram[0]);
            I::load_data_section(&mut *ram_start, ram_past_end, &rom[0]);
        }

        expect!(unsafe { ram[0].assume_init() }).to_equal(original_value);
    }

    fn load_data_section__called__expect_ram_is_loaded_up_to_past_end_with_given_rom_contents<I: DataSectionInitialisation>() {
        let mut ram = any_vec_filled_using(2..1024, || MaybeUninit::new(any_usize()));
        let rom = any_vec_filled_using(size_matching(ram.len()), any_usize);
        let (head, tail) = ram.split_at_mut(1);
        let all_excluding_past_end = unsafe {
            I::load_data_section(&mut head[0], tail.last().unwrap(), &rom[0]);
            ram.pop();
            ram.iter().map(|x| x.assume_init())
        };

        expect!(all_excluding_past_end).to_equal_iterators(&mut rom[0..rom.len() - 1].iter().cloned());
    }

    fn size_matching(length: usize) -> RangeInclusive<usize> {
        length..=length
    }

    fn load_data_section__called__expect_ram_past_end_is_not_modified<I: DataSectionInitialisation>() {
        let mut ram = any_vec_filled_using(2..1024, || MaybeUninit::new(any_usize()));
        let original_value = unsafe { ram.last().unwrap().assume_init() };
        let rom = any_vec_filled_using(size_matching(ram.len()), || any_usize_except(original_value));
        let (head, tail) = ram.split_at_mut(1);
        let last_word = unsafe {
            I::load_data_section(&mut head[0], tail.last().unwrap(), &rom[0]);
            ram.last().unwrap().assume_init()
        };

        expect!(last_word).to_equal(original_value);
    }

    mod bss_section_initialiser_with_checks {
        use crate::bootstrapping::rust::DataSectionInitialiserWithChecks;

        #[test]
        fn load_data_section__called_with_ram_start_equal_to_ram_past_end__expect_ram_is_not_modified() {
            super::load_data_section__called_with_ram_start_equal_to_ram_past_end__expect_ram_is_not_modified::<DataSectionInitialiserWithChecks>()
        }

        #[test]
        fn load_data_section__called__expect_ram_is_loaded_up_to_past_end_with_given_rom_contents() {
            super::load_data_section__called__expect_ram_is_loaded_up_to_past_end_with_given_rom_contents::<DataSectionInitialiserWithChecks>()
        }

        #[test]
        fn load_data_section__called__expect_ram_past_end_is_not_modified() {
            super::load_data_section__called__expect_ram_past_end_is_not_modified::<DataSectionInitialiserWithChecks>()
        }
    }

    mod bss_section_initialiser_without_checks {
        use crate::bootstrapping::rust::DataSectionInitialiserWithoutChecks;

        #[test]
        fn load_data_section__called_with_ram_start_equal_to_ram_past_end__expect_ram_is_not_modified() {
            super::load_data_section__called_with_ram_start_equal_to_ram_past_end__expect_ram_is_not_modified::<DataSectionInitialiserWithoutChecks>()
        }

        #[test]
        fn load_data_section__called__expect_ram_is_loaded_up_to_past_end_with_given_rom_contents() {
            super::load_data_section__called__expect_ram_is_loaded_up_to_past_end_with_given_rom_contents::<DataSectionInitialiserWithoutChecks>()
        }

        #[test]
        fn load_data_section__called__expect_ram_past_end_is_not_modified() {
            super::load_data_section__called__expect_ram_past_end_is_not_modified::<DataSectionInitialiserWithoutChecks>()
        }
    }
}
