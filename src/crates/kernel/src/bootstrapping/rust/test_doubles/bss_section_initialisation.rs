#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use core::mem::MaybeUninit;

use crate::bootstrapping::rust::BssSectionInitialisation;

use crate::test_doubles::Dummy;

unsafe impl BssSectionInitialisation for Dummy {
    #[doc = docs::side_by_side_md!("Dummy.fill_bss_section")]
    unsafe fn fill_bss_section(_start: &mut MaybeUninit<usize>, _past_end: &MaybeUninit<usize>, _fill_value: u8) { }
}
