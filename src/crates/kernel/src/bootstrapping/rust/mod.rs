mod bss_section_initialiser;
pub use bss_section_initialiser::*;

mod data_section_initialiser;
pub use data_section_initialiser::*;

mod runtime_bootstrapping;
pub use runtime_bootstrapping::*;

#[cfg(not(feature = "no_bootstrapping_bss_data_bounds_checks"))]
mod runtime_initialisers {
    pub type BssSectionInitialiser = super::BssSectionInitialiserWithChecks;
    pub type DataSectionInitialiser = super::DataSectionInitialiserWithChecks;
}

#[cfg(feature = "no_bootstrapping_bss_data_bounds_checks")]
mod runtime_initialisers {
    pub type BssSectionInitialiser = super::BssSectionInitialiserWithoutChecks;
    pub type DataSectionInitialiser = super::DataSectionInitialiserWithoutChecks;
}

pub unsafe fn initialise<R: RuntimeBootstrapping>() {
    // TODO: .bss, .data, etc.
    // This is just temporary debugging below here...a better way of doing this is to pass the types as 'R', to facilitate testing
    unsafe {
        R::initialise_bss_sections_using(&runtime_initialisers::BssSectionInitialiser{});
        R::initialise_data_sections_using(&runtime_initialisers::DataSectionInitialiser{});
    }
}
