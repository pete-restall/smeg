use super::board::bootstrapping::rust::McuMemoryBootstrapper;

use smeg_kernel::bootstrapping::rust::{DefaultPanicBootstrapper, RuntimeBootstrapping};

pub struct Rust;

unsafe impl RuntimeBootstrapping for Rust {
    type BssSectionInitialiser = selected::BssSectionInitialiser;
    type DataSectionInitialiser = selected::DataSectionInitialiser;
    type McuMemoryBootstrapper = McuMemoryBootstrapper;
    type PanicBootstrapper = DefaultPanicBootstrapper;
}

#[cfg(feature = "smeg-kernel-no_bootstrapping_bss_data_safety_checks")]
mod selected {
    use smeg_kernel::bootstrapping::rust;
    pub type BssSectionInitialiser = rust::BssSectionInitialiserWithoutChecks;
    pub type DataSectionInitialiser = rust::DataSectionInitialiserWithoutChecks;
}

#[cfg(not(feature = "smeg-kernel-no_bootstrapping_bss_data_safety_checks"))]
mod selected {
    use smeg_kernel::bootstrapping::rust;
    pub type BssSectionInitialiser = rust::BssSectionInitialiserWithChecks;
    pub type DataSectionInitialiser = rust::DataSectionInitialiserWithChecks;
}
