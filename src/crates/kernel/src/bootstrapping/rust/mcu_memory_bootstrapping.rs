use super::{BssSectionInitialisation, DataSectionInitialisation};

// TODO: Missing docs !
pub unsafe trait McuMemoryBootstrapping {
    unsafe fn bootstrap_bss_sections_using<I: BssSectionInitialisation>();
    unsafe fn bootstrap_data_sections_using<I: DataSectionInitialisation>();
}
