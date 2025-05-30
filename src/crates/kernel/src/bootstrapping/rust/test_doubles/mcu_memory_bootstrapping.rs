#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use crate::bootstrapping::rust::{BssSectionInitialisation, DataSectionInitialisation, McuMemoryBootstrapping};

use crate::test_doubles::Dummy;

unsafe impl McuMemoryBootstrapping for Dummy {
    #[doc = docs::side_by_side_md!("Dummy.bootstrap_bss_sections_using")]
    unsafe fn bootstrap_bss_sections_using<I: BssSectionInitialisation>() { }

    #[doc = docs::side_by_side_md!("Dummy.bootstrap_data_sections_using")]
    unsafe fn bootstrap_data_sections_using<I: DataSectionInitialisation>() { }
}
