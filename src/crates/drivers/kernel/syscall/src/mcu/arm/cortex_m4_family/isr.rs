use core::borrow::BorrowMut;

use smeg_kernel::errors::{KernelErrorCode, ResultToUsizeResultConversion, TaggedError};

use smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines;
use smeg_mcu_arm_cortex_m4_family::interrupts::{HasIsrBasicStackFrameMut, IsrContextImpl};
pub use smeg_mcu_arm_cortex_m4_family::interrupts::IsrVectorTableBuilder;

use crate::Dependencies;
use crate::isr::{DefaultSyscallIsrDispatcher, SyscallIsrDispatcher};

isr_fn_trampolines! {
    fn on_sv_call_isr_trampoline<Dependencies>() -> on_sv_call_isr<DefaultSyscallIsrDispatcher<D>>() -> "thread_main" /* TODO: "thread_process" or even a new option, to allow context-switching */;
}

pub trait IsrContext: smeg_kernel::interrupts::IsrContext + From<IsrContextImpl> + BorrowMut<IsrContextImpl> { }
impl<T: smeg_kernel::interrupts::IsrContext + From<IsrContextImpl> + BorrowMut<IsrContextImpl>> IsrContext for T { }

pub const fn collect_isr_vectors<D: Dependencies>(isrs: IsrVectorTableBuilder) -> IsrVectorTableBuilder {
    IsrVectorTableBuilder {
        sv_call: Some(on_sv_call_isr_trampoline::<D>),
        ..isrs
    }
}

unsafe fn on_sv_call_isr<D: Dependencies, S: SyscallIsrDispatcher<D>>(isr_context: &mut D::IsrContext) {
    let (id, args) = unsafe {
        let stack_frame = isr_context.borrow_mut().basic_stack_frame_mut();
        (stack_frame.r0, stack_frame.r1)
    };

    unsafe {
        let result = S::dispatch_syscall(isr_context, id, args).as_usize_result();
        let stack_frame = isr_context.borrow_mut().basic_stack_frame_mut();
        stack_frame.r1 = *(&raw const result as *const usize); // TODO: encapsulate this abomination somewhere inside the KernelError implementation
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use core::marker::PhantomData;
    use core::num::NonZero;

    use fluent_test::prelude::*;

    use smeg_kernel::syscalls::SyscallResult;
    use smeg_kernel::test_doubles::StubFor;

    use smeg_mcu_arm_cortex_m4_family::interrupts::{HasIsrBasicStackFrame};
    use smeg_mcu_arm_cortex_m4_family::interrupts::test_doubles::{Dummy, Stub};
    use smeg_mcu_arm_cortex_m4_family::interrupts::test_doubles::isr_context::StubIsrContext;

    use smeg_testing_host_utils::integers::{any_usize, any_usize_except};

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
    fn collect_isr_vectors__called__expect_same_vectors_excluding_sv_call() {
        let original_isrs = IsrVectorTableBuilder::from(Stub);
        let original_isrs_excluding_sv_call = IsrVectorTableBuilder { sv_call: None, ..original_isrs };

        let added_isrs = collect_isr_vectors::<Dummy>(original_isrs.clone());
        let added_isrs_excluding_sv_call = IsrVectorTableBuilder { sv_call: None, ..added_isrs };

        expect!(added_isrs_excluding_sv_call == original_isrs_excluding_sv_call).to_be_true();
    }

    #[test]
    fn collect_isr_vectors__called__expect_sv_call_isr_is_added() {
        let original_isrs = IsrVectorTableBuilder::from(Stub);
        let added_isrs = collect_isr_vectors::<Dummy>(original_isrs.clone());
        expect!(added_isrs.sv_call).to_equal(Some(on_sv_call_isr_trampoline::<Dummy>));
    }

    #[test]
    fn on_sv_call_isr__called_when_dispatcher_returns_ok__expect_r1_on_stack_is_zero() {
        let ok: SyscallResult = Ok(());
        on_sv_call_isr__called__expect_r1_on_stack_is_usize_equivalent_of_result_returned_by_dispatcher(ok);
    }

    fn on_sv_call_isr__called__expect_r1_on_stack_is_usize_equivalent_of_result_returned_by_dispatcher(result: SyscallResult) {
        let error_as_usize = match result {
            Ok(()) => 0,
            Err(error) => NonZero::<usize>::from(error).get()
        };

        let mut isr_context = StubIsrContext::from(StubFor { value: result });
        unsafe {
            BorrowMut::<IsrContextImpl>::borrow_mut(&mut isr_context).basic_stack_frame_mut().r1 = any_usize_except(error_as_usize);
        }

        struct StubDispatcherForResult;
        unsafe impl SyscallIsrDispatcher<StubDependenciesFor<StubIsrContext<SyscallResult>>> for StubDispatcherForResult {
            unsafe fn dispatch_syscall(isr_context: &mut StubIsrContext<SyscallResult>, _id: usize, _args: usize) -> SyscallResult {
                isr_context.stubbed_with.unwrap()
            }
        }

        unsafe { on_sv_call_isr::<_, StubDispatcherForResult>(&mut isr_context); }

        let r1 = unsafe { BorrowMut::<IsrContextImpl>::borrow_mut(&mut isr_context).basic_stack_frame().r1 };
        expect!(r1).to_equal(error_as_usize);
    }

    #[test]
    fn on_sv_call_isr__called_when_dispatcher_returns_err__expect_r1_on_stack_is_equivalent_usize() {
        let error = Err(smeg_kernel::errors::test_doubles::any_kernel_error());
        on_sv_call_isr__called__expect_r1_on_stack_is_usize_equivalent_of_result_returned_by_dispatcher(error);
    }

    #[test]
    fn on_sv_call_isr__called__expect_r0_on_stack_is_passed_to_dispatcher_as_id() {
        let mut isr_context = StubIsrContext::from(StubFor { value: any_usize() });
        unsafe {
            let stacked_r0 = isr_context.stubbed_with;
            let stack_frame = BorrowMut::<IsrContextImpl>::borrow_mut(&mut isr_context).basic_stack_frame_mut();
            stack_frame.r0 = stacked_r0.unwrap();
        };

        struct MockDispatcher;
        unsafe impl SyscallIsrDispatcher<StubDependenciesFor<StubIsrContext<usize>>> for MockDispatcher {
            unsafe fn dispatch_syscall(isr_context: &mut StubIsrContext<usize>, id: usize, _args: usize) -> SyscallResult {
                let expected_id = isr_context.stubbed_with.unwrap();
                expect!(id).to_equal(expected_id);
                Ok(())
            }
        }

        unsafe { on_sv_call_isr::<_, MockDispatcher>(&mut isr_context); }
    }

    #[test]
    fn on_sv_call_isr__called__expect_r1_on_stack_is_passed_to_dispatcher_as_args() {
        let mut isr_context = StubIsrContext::from(StubFor { value: any_usize() });
        unsafe {
            let stacked_r1 = isr_context.stubbed_with.unwrap();
            let stack_frame = BorrowMut::<IsrContextImpl>::borrow_mut(&mut isr_context).basic_stack_frame_mut();
            stack_frame.r1 = stacked_r1;
        };

        struct MockDispatcher;
        unsafe impl SyscallIsrDispatcher<StubDependenciesFor<StubIsrContext<usize>>> for MockDispatcher {
            unsafe fn dispatch_syscall(isr_context: &mut StubIsrContext<usize>, _id: usize, args: usize) -> SyscallResult {
                let expected_args = isr_context.stubbed_with.unwrap();
                expect!(args).to_equal(expected_args);
                Ok(())
            }
        }

        unsafe { on_sv_call_isr::<_, MockDispatcher>(&mut isr_context); }
    }
}
