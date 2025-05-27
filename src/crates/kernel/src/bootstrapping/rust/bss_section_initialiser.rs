use core::mem::MaybeUninit;

//use smeg_kernel_procmacro::link_doc;

use crate::despair;
use crate::errors::KernelErrorCode;

//#[link_doc]

//#[link_doc("BssSectionInitialiser")]
pub unsafe trait BssSectionInitialiser {
    //#[link_doc("BssSectionInitialiser::fill_bss_section")]
    unsafe fn fill_bss_section(&self, start: &mut MaybeUninit<usize>, past_end: &MaybeUninit<usize>, fill_value: u8);
}

//#[link_doc("BssSectionInitialiserWithChecks")]
pub struct BssSectionInitialiserWithChecks;

unsafe impl BssSectionInitialiser for BssSectionInitialiserWithChecks {
    //#[link_doc("BssSectionInitialiserWithChecks::fill_bss_section")]
    unsafe fn fill_bss_section(&self, start: &mut MaybeUninit<usize>, past_end: &MaybeUninit<usize>, fill_value: u8) {
        if start.as_ptr() > past_end.as_ptr() {
            despair!(with(KernelErrorCode::LinkerScriptDespair), because("Linker-supplied section pointers for .bss are corrupt"));
        }

        unsafe {
            BssSectionInitialiserWithoutChecks.fill_bss_section(start, past_end, fill_value)
        }
    }
}

//#[link_doc("BssSectionInitialiserWithoutChecks")]
pub struct BssSectionInitialiserWithoutChecks;

unsafe impl BssSectionInitialiser for BssSectionInitialiserWithoutChecks {
    //#[link_doc("BssSectionInitialiserWithoutChecks::fill_bss_section")]
    unsafe fn fill_bss_section(&self, start: &mut MaybeUninit<usize>, past_end: &MaybeUninit<usize>, fill_value: u8) {
        unsafe {
            let bss_size_words = past_end.as_ptr().offset_from(start.as_ptr());
            core::ptr::write_bytes(start.as_mut_ptr(), fill_value, bss_size_words as usize);
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use core::mem::MaybeUninit;

    use fluent_test::prelude::*;

    use crate::bootstrapping::rust::BssSectionInitialiser;

    use smeg_testing_host_utils::integers::{any_u8, any_u8_except, any_usize};
    use smeg_testing_host_utils::seq::OnlyContainCopyableMatchers;
    use smeg_testing_host_utils::vec::any_vec_filled_using;

    fn fill_bss_section__called_with_start_equal_to_past_end__expect_block_is_not_filled<T: BssSectionInitialiser>(initialiser: T) {
        let original_value = any_usize();
        let fill_value = any_u8_except(original_value as u8);
        let mut block = [MaybeUninit::<usize>::new(original_value); 1];
        unsafe {
            let (start, past_end) = (&raw mut block[0], &block[0]);
            initialiser.fill_bss_section(&mut *start, past_end, fill_value);
        }

        expect!(unsafe { block[0].assume_init() }).to_equal(original_value);
    }

    fn fill_bss_section__called__expect_block_is_filled_up_to_past_end_with_given_byte<T: BssSectionInitialiser>(initialiser: T) {
        let fill_value = any_fill_value();
        let fill_value_as_usize = usize_packed_with(fill_value);
        let mut block = any_vec_filled_using(2..1024, || MaybeUninit::<usize>::new(any_usize()));
        let (head, tail) = block.split_at_mut(1);
        let all_excluding_past_end = unsafe {
            initialiser.fill_bss_section(&mut head[0], tail.last().unwrap(), fill_value);
            block.pop();
            block.iter().map(|x| x.assume_init())
        };

        expect!(all_excluding_past_end).to_only_contain(fill_value_as_usize);
    }

    fn any_fill_value() -> u8 {
        any_u8()
    }

    fn usize_packed_with(fill_value: u8) -> usize {
        (0..usize::BITS / 8).into_iter().fold(0, |x, _| (x << 8) | (fill_value as usize))
    }

    fn fill_bss_section__called__expect_block_past_end_is_not_filled<T: BssSectionInitialiser>(initialiser: T) {
        let mut block = any_vec_filled_using(2..1024, || MaybeUninit::<usize>::new(any_usize()));
        let original_value = unsafe { block.last().unwrap().assume_init() };
        let fill_value = any_u8_except(original_value as u8);
        let (head, tail) = block.split_at_mut(1);
        let last_word = unsafe {
            initialiser.fill_bss_section(&mut head[0], tail.last().unwrap(), fill_value);
            block.last().unwrap().assume_init()
        };

        expect!(last_word).to_equal(original_value);
    }

    mod bss_section_initialiser_with_checks {
        use crate::bootstrapping::rust::BssSectionInitialiserWithChecks;

        #[test]
        fn fill_bss_section__called_with_start_equal_to_past_end__expect_block_is_not_filled() {
            super::fill_bss_section__called_with_start_equal_to_past_end__expect_block_is_not_filled(BssSectionInitialiserWithChecks)
        }

        #[test]
        fn fill_bss_section__called__expect_block_is_filled_up_to_past_end_with_given_byte() {
            super::fill_bss_section__called__expect_block_is_filled_up_to_past_end_with_given_byte(BssSectionInitialiserWithChecks)
        }

        #[test]
        fn fill_bss_section__called__expect_block_past_end_is_not_filled() {
            super::fill_bss_section__called__expect_block_past_end_is_not_filled(BssSectionInitialiserWithChecks)
        }
    }

    mod bss_section_initialiser_without_checks {
        use crate::bootstrapping::rust::BssSectionInitialiserWithoutChecks;

        #[test]
        fn fill_bss_section__called_with_start_equal_to_past_end__expect_block_is_not_filled() {
            super::fill_bss_section__called_with_start_equal_to_past_end__expect_block_is_not_filled(BssSectionInitialiserWithoutChecks)
        }

        #[test]
        fn fill_bss_section__called__expect_block_is_filled_up_to_past_end_with_given_byte() {
            super::fill_bss_section__called__expect_block_is_filled_up_to_past_end_with_given_byte(BssSectionInitialiserWithoutChecks)
        }

        #[test]
        fn fill_bss_section__called__expect_block_past_end_is_not_filled() {
            super::fill_bss_section__called__expect_block_past_end_is_not_filled(BssSectionInitialiserWithoutChecks)
        }
    }
}
