use super::{BssSectionInitialiser, DataSectionInitialiser};

pub unsafe trait RuntimeBootstrapping {
    unsafe fn initialise_bss_sections_using<I: BssSectionInitialiser>(initialiser: &I);
    unsafe fn initialise_data_sections_using<I: DataSectionInitialiser>(initialiser: &I);
}
