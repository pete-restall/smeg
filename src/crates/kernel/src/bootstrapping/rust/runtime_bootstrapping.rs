use crate::{caller, docs};

use super::{BssSectionInitialisation, DataSectionInitialisation, McuMemoryBootstrapping, PanicBootstrapping};

#[doc = docs::side_by_side_md!("RuntimeBootstrapping")]
pub unsafe trait RuntimeBootstrapping {
    type BssSectionInitialiser: BssSectionInitialisation;
    type DataSectionInitialiser: DataSectionInitialisation;
    type McuMemoryBootstrapper: McuMemoryBootstrapping;
    type PanicBootstrapper: PanicBootstrapping;

    #[doc = docs::side_by_side_md!("RuntimeBootstrapping.bootstrap")]
    unsafe fn bootstrap<K: caller::RestrictedToKernel>() {
        unsafe {
            Self::McuMemoryBootstrapper::bootstrap_bss_sections_using::<Self::BssSectionInitialiser>();
            Self::McuMemoryBootstrapper::bootstrap_data_sections_using::<Self::DataSectionInitialiser>();
        }

        Self::PanicBootstrapper::bootstrap::<K>();
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
            static INITIALISED_COUNTER: Cell<i32> = Cell::new(1);
            static INITIALISED_BSS: Cell<i32> = Cell::new(0);
            static INITIALISED_DATA: Cell<i32> = Cell::new(0);
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
            type PanicBootstrapper = Dummy;
        }

        unsafe { RuntimeBootstrapper::bootstrap::<caller::IsKernel>(); }

        expect!((INITIALISED_BSS.get(), INITIALISED_DATA.get())).to_equal((1, 2));
    }

    #[test]
    fn bootstrap__called__expect_memory_is_bootstrapped_before_panic() {
        thread_local! {
            static BOOTSTRAPPED_COUNTER: Cell<i32> = Cell::new(1);
            static BOOTSTRAPPED_MEMORY: Cell<i32> = Cell::new(0);
            static BOOTSTRAPPED_PANIC: Cell<i32> = Cell::new(0);
        }

        struct MockMcuMemoryBootstrapper;
        unsafe impl McuMemoryBootstrapping for MockMcuMemoryBootstrapper {
            unsafe fn bootstrap_bss_sections_using<I: BssSectionInitialisation>() {
                increment_bootstrapped_memory_counter();
            }

            unsafe fn bootstrap_data_sections_using<I: DataSectionInitialisation>() {
                increment_bootstrapped_memory_counter();
            }
        }

        fn increment_bootstrapped_memory_counter() {
            let counter = BOOTSTRAPPED_COUNTER.get();
            BOOTSTRAPPED_MEMORY.set(counter);
            BOOTSTRAPPED_COUNTER.replace(counter + 1);
        }

        struct MockPanicBootstrapper;
        impl PanicBootstrapping for MockPanicBootstrapper {
            fn bootstrap<K: caller::RestrictedToKernel>() {
                let counter = BOOTSTRAPPED_COUNTER.get();
                BOOTSTRAPPED_PANIC.set(counter);
                BOOTSTRAPPED_COUNTER.replace(counter + 1);
            }
        }

        struct RuntimeBootstrapper;
        unsafe impl RuntimeBootstrapping for RuntimeBootstrapper {
            type BssSectionInitialiser = Dummy;
            type DataSectionInitialiser = Dummy;
            type McuMemoryBootstrapper = MockMcuMemoryBootstrapper;
            type PanicBootstrapper = MockPanicBootstrapper;
        }

        unsafe { RuntimeBootstrapper::bootstrap::<caller::IsKernel>(); }

        expect!(BOOTSTRAPPED_MEMORY.get()).to_be_less_than(BOOTSTRAPPED_PANIC.get());
    }
}
