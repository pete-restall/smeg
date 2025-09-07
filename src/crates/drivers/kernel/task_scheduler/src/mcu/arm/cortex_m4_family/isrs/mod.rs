use smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines;
pub use smeg_mcu_arm_cortex_m4_family::interrupts::IsrContext;
pub use smeg_mcu_arm_cortex_m4_family::interrupts::IsrVectorTableBuilder;

use crate::Dependencies;

mod pend_sv;
use pend_sv::on_pend_sv_isr;

isr_fn_trampolines! {
    fn on_pend_sv_isr_trampoline<Dependencies>() -> on_pend_sv_isr() -> "thread_process" /* TODO: needs a new option, to allow context-switching */;
}

pub const fn collect_isr_vectors<D: Dependencies>(isrs: IsrVectorTableBuilder) -> IsrVectorTableBuilder {
    IsrVectorTableBuilder {
        pend_sv: Some(on_pend_sv_isr_trampoline::<D>),
        ..isrs
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use core::marker::PhantomData;
    use core::num::NonZero;

    use fluent_test::prelude::*;

    use smeg_kernel::test_doubles::StubFor;

    use smeg_mcu_arm_cortex_m4_family::interrupts::{HasIsrBasicStackFrame};
    use smeg_mcu_arm_cortex_m4_family::interrupts::test_doubles::{Dummy, Stub};
    use smeg_mcu_arm_cortex_m4_family::interrupts::test_doubles::isr_context::StubIsrContext;

    use smeg_testing_host_utils::integers::{any_usize, any_usize_except};

    use crate::SyscallResult;

    use super::*;

    struct StubDependenciesFor<I: IsrContext> {
        _unused: PhantomData<I>
    }

    impl<I: IsrContext> Dependencies for StubDependenciesFor<I> {
        type IsrContext = I;
    }

    impl Dependencies for Dummy {
        type IsrContext = Dummy;
    }

    #[unsafe(no_mangle)]
    unsafe extern "C" fn _reset_handler() -> ! {
        panic!("Aborting because the _reset_handler stub should never be called");
    }

    #[test]
    fn collect_isr_vectors__called__expect_same_vectors_excluding_pend_sv() {
        let original_isrs = IsrVectorTableBuilder::from(Stub);
        let original_isrs_excluding_pend_sv = IsrVectorTableBuilder { pend_sv: None, ..original_isrs };

        let added_isrs = collect_isr_vectors::<Dummy>(original_isrs.clone());
        let added_isrs_excluding_pend_sv = IsrVectorTableBuilder { pend_sv: None, ..added_isrs };

        expect!(added_isrs_excluding_pend_sv == original_isrs_excluding_pend_sv).to_be_true();
    }

    #[test]
    fn collect_isr_vectors__called__expect_pend_sv_isr_is_added() {
        let original_isrs = IsrVectorTableBuilder::from(Stub);
        let added_isrs = collect_isr_vectors::<Dummy>(original_isrs.clone());
        expect!(added_isrs.pend_sv).to_equal(Some(on_pend_sv_isr_trampoline::<Dummy>));
    }
}
