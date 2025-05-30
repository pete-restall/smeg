#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use core::mem::MaybeUninit;

use crate::bootstrapping::rust::DataSectionInitialisation;

use crate::test_doubles::Dummy;

unsafe impl DataSectionInitialisation for Dummy {
    #[doc = docs::side_by_side_md!("Dummy.load_data_section")]
    unsafe fn load_data_section(_ram_start: &mut MaybeUninit<usize>, _ram_past_end: &MaybeUninit<usize>, _rom_start: &usize) { }
}
