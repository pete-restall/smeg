use core::mem::MaybeUninit;

use smeg_kernel::interrupts::IsrContext;
use smeg_kernel::syscalls::SyscallResult;

use super::{SyscallIsrContext, SyscallIsrHandler};

pub type SyscallIsrTrampolinePtr<I: IsrContext> = unsafe fn(isr_context: &mut I, args: usize) -> SyscallResult;

pub unsafe trait SyscallIsrTrampoline<I: IsrContext> {
    unsafe fn on_syscall(isr_context: &mut I, args: usize) -> SyscallResult;
}

unsafe impl<H: SyscallIsrHandler> SyscallIsrTrampoline<H::IsrContext> for H {
    unsafe fn on_syscall(isr_context: &mut H::IsrContext, args: usize) -> SyscallResult {
/*
        // Alignment can be checked here; the size can too if the IsrContext is passed in so we can retrieve the current task's stack
        // (or heap) information, which will be the case in the actual implementation.
        if align_of::<H::Args>() > 1 && args & (align_of::<H::Args>() - 1) != 0 {
            return Err(123); // Some sort of error code for an unaligned access
        }

        let mut context = SyscallIsrContext::new(
            isr_context,
            unsafe { &mut *(args as *mut MaybeUninit<H::Args>) }); // the 'new' function will be pub(crate) so SyscallIsrContext cannot be created outside of this crate
        <H as SyscallIsrHandler>::on_syscall(&mut context)
*/
        SyscallResult::Ok(())
    }
}
