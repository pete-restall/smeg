use core::marker::PhantomData;
use core::mem::MaybeUninit;

use smeg_kernel::interrupts::IsrContext;
use smeg_kernel::syscalls::SyscallResult;

use crate::Dependencies;

use super::SyscallIsrTrampolinePtr;

pub unsafe trait SyscallIsrDispatcher<D: Dependencies> {
    unsafe fn dispatch_syscall(isr_context: &mut D::IsrContext, id: usize, args: usize) -> SyscallResult;
}

pub struct DefaultSyscallIsrDispatcher<D: Dependencies> {
    _dependencies: PhantomData<D>
}

unsafe impl<D: Dependencies> SyscallIsrDispatcher<D> for DefaultSyscallIsrDispatcher<D> {
    unsafe fn dispatch_syscall(isr_context: &mut D::IsrContext, id: usize, args: usize) -> SyscallResult {
        const {
            assert!(
                size_of::<SyscallIsrTrampolinePtr<D::IsrContext>>() == size_of::<usize>(),
                "This code makes the assumption that a function pointer (SyscallIsrTrampolinePtr) can fit into a single machine word");

            assert!(
                size_of::<SyscallIsrTrampolinePtr<D::IsrContext>>() == align_of::<SyscallIsrTrampolinePtr<D::IsrContext>>(),
                r#"This code makes the assumption that SyscallIsrTrampolinePtr size and alignment are the same (ie. a single field) to allow a quick
                runtime check (ie. single comparison), otherwise it is possible to pass something bad with correct alignment but doesn't point to the
                start of the struct, which may not be a simple power-of-two (as alignment is guaranteed to be).  This scenario would preclude a single
                quick AND mask test"#);
        }

        todo!();

/*
        if id & (align_of::<SyscallIsrTrampolinePtr<I>>() - 1) != 0 {
            return Err(KernelError::from(TaggedError::new(KernelErrorCode::UnknownSyscall, error_tag!())));
        }


        let trampoline = id as *const SyscallIsrTrampolinePtr<I>; // UB only happens on dereferencing the pointer so we're able to check its bounds first
        if
            (trampoline < &raw const __LINKER_SYSCALLS_ISR_TRAMPOLINES_START) ||
            (trampoline > unsafe { (&raw const __LINKER_SYSCALLS_ISR_TRAMPOLINES_PAST_END).offset(-1) }) {

            Err(KernelError::from(TaggedError::new(KernelErrorCode::UnknownSyscall, error_tag!())))
        }

        unsafe { (*trampoline)(isr_context, args) }
*/
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
/*
    use fluent_test::prelude::*;
    use smeg_kernel::{errors::KernelErrorCode, test_doubles::Dummy};
    use smeg_mcu_arm_cortex_m4_family::interrupts::test_doubles::isr_context;

    use super::*;

    #[test]
    fn dispatch_syscall__called_with_unaligned_id__expect_unknown_syscall_err_is_returned() {
        struct Driver;
        let trampoline_table = [dummy_trampoline];
        impl Dependencies for Driver {
            type IsrContext = Dummy;
            fn trampoline_vector_table() -> Option<&[SyscallIsrTrampolinePtr<Dummy>]> { &trampoline_table }
        }

        let mut isr_context = Dummy;
        let unaligned_id = &raw const trampoline_table as usize - 1; // -2, -3, +1, +2, +3
        let args = Dummy;
        let result = unsafe { DefaultSyscallIsrDispatcher::<Driver>::dispatch_syscall(isr_context, unaligned_id, &raw const args as usize) };
        expect!(result.unwrap_err().code).to_equal(KernelErrorCode::UnknownSyscall);
    }

    unsafe fn dummy_trampoline(_isr_context: &mut Dummy, _args: usize) -> SyscallResult { Ok(()) }
*/
}

#[cfg(any(test, feature = "test_doubles"))]
pub mod test_doubles {
    use smeg_kernel::syscalls::SyscallResult;

    use crate::isr::SyscallIsrDispatcher;
/*
    pub struct StubSyscallIsrDispatcherFor<C: IsrContext, F: FnOnce(&mut C,  usize, usize) -> SyscallResult> {
    }

    unsafe impl<C, F> SyscallIsrDispatcher<C> for StubSyscallIsrDispatcherFor<C, F>
        where C: IsrContext, F: FnOnce(&mut C,  usize, usize) -> SyscallResult {
        unsafe fn dispatch_syscall(isr_context: &mut C, id: usize, args: usize) -> SyscallResult {
            F::call_once(self, args)
        }
    }
*/
// implement the SyscalIsrDispatcher trait...
//    pub unsafe fn on_syscall_isr<C: IsrContext>(isr_context: &mut C, id: usize, args: usize) -> SyscallResult {
//    }
}
