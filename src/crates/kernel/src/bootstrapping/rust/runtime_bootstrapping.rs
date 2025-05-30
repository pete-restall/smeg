use crate::docs;

use super::{BssSectionInitialisation, DataSectionInitialisation, McuMemoryBootstrapping};

#[doc = docs::side_by_side_md!("RuntimeBootstrapping")]
pub unsafe trait RuntimeBootstrapping {
    type BssSectionInitialiser: BssSectionInitialisation;
    type DataSectionInitialiser: DataSectionInitialisation;
    type McuMemoryBootstrapper: McuMemoryBootstrapping;

    #[doc = docs::side_by_side_md!("RuntimeBootstrapping.bootstrap")]
    unsafe fn bootstrap() {
        unsafe {
            Self::McuMemoryBootstrapper::bootstrap_bss_sections_using::<Self::BssSectionInitialiser>();
            Self::McuMemoryBootstrapper::bootstrap_data_sections_using::<Self::DataSectionInitialiser>();
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::cell::Cell;

    use fluent_test::prelude::*;

    use crate::test_doubles::Dummy;

    use super::*;

    #[test]
    fn bootstrap__called__expect_bss_sections_are_bootstrapped_before_data_sections() {
        thread_local! {
            static INITIALISED_COUNTER: Cell<isize> = Cell::new(1);
            static INITIALISED_BSS: Cell<isize> = Cell::new(0);
            static INITIALISED_DATA: Cell<isize> = Cell::new(0);
        }

        struct MockMcuMemoryBootstrapper;
        unsafe impl McuMemoryBootstrapping for MockMcuMemoryBootstrapper {
            unsafe fn bootstrap_bss_sections_using<I: BssSectionInitialisation>() {
                let counter = INITIALISED_COUNTER.get();
                INITIALISED_BSS.set(counter);
                INITIALISED_COUNTER.replace(counter + 1);
            }

            unsafe fn bootstrap_data_sections_using<I: DataSectionInitialisation>() {
                let counter = INITIALISED_COUNTER.get();
                INITIALISED_DATA.set(counter);
                INITIALISED_COUNTER.replace(counter + 1);
            }
        }

        struct RuntimeBootstrapper;
        unsafe impl RuntimeBootstrapping for RuntimeBootstrapper {
            type BssSectionInitialiser = Dummy;
            type DataSectionInitialiser = Dummy;
            type McuMemoryBootstrapper = MockMcuMemoryBootstrapper;
        }

        unsafe { RuntimeBootstrapper::bootstrap(); }

        expect!((INITIALISED_BSS.get(), INITIALISED_DATA.get())).to_equal((1, 2));
    }
}
