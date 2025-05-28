#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

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

#[doc = docs::side_by_side_md!("initialise")]
pub unsafe fn initialise<R: RuntimeBootstrapping>() {
    unsafe {
        R::initialise_bss_sections_using(&runtime_initialisers::BssSectionInitialiser{});
        R::initialise_data_sections_using(&runtime_initialisers::DataSectionInitialiser{});
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::cell::Cell;

    use fluent_test::prelude::*;

    use super::*;

    #[test]
    fn initialise__called__expect_bss_sections_are_initialised_before_data_sections() {
        struct MockBootstrapper;

        thread_local! {
            static INITIALISED_COUNTER: Cell<isize> = Cell::new(1);
            static INITIALISED_BSS: Cell<isize> = Cell::new(0);
            static INITIALISED_DATA: Cell<isize> = Cell::new(0);
        }

        unsafe impl RuntimeBootstrapping for MockBootstrapper {
            unsafe fn initialise_bss_sections_using<I: BssSectionInitialiser>(_initialiser: &I) {
                let counter = INITIALISED_COUNTER.get();
                INITIALISED_BSS.set(counter);
                INITIALISED_COUNTER.replace(counter + 1);
            }

            unsafe fn initialise_data_sections_using<I: DataSectionInitialiser>(_initialiser: &I) {
                let counter = INITIALISED_COUNTER.get();
                INITIALISED_DATA.set(counter);
                INITIALISED_COUNTER.replace(counter + 1);
            }
        }

        unsafe { initialise::<MockBootstrapper>(); }

        expect!((INITIALISED_BSS.get(), INITIALISED_DATA.get())).to_equal((1, 2));
    }
}
