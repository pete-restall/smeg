use core::sync::atomic::{compiler_fence, Ordering};

use smeg_kernel::errors::{UsizeResult, UsizeResultConversions};
use smeg_kernel::syscalls::SyscallResult;

use crate::{SyscallArgs, SyscallInvocation};

#[cfg(not(feature = "no_default_syscall_invocation"))]
impl<T: SyscallArgs> SyscallInvocation for T {
    #[inline(always)]
    // #[doc = docs::side_by_side_md!("SyscallInvocation.invoke_syscall")]
    fn invoke_syscall(&mut self) -> SyscallResult {
/*
        // This will be the ARM 'svc' or something... r0 is T::syscall_id and r1 can be a pointer to self - the handler needs to verify
        // that r1 (and r1 + sizeof<r1>) is within the stack boundaries, plus alignof<r1> == alignof<T> before it uses it...
        // HOWEVER - this can still introduce UB into the kernel if something passes a pointer to a block of uninitialised RAM, for example.  The
        // alternative is to always get the arguments from a block stored in the TCB, but then this entails allowing the userspace code to access
        // the TCB !  Neither is good, but the stack approach is better.  A third approach might be to use a static location known to both userspace
        // and kernelspace (difficult, due to number of cores - no thread-local storage...) but that still doesn't address the underlying issue that
        // the userspace code could write utter junk to the contents of the buffer (for example, a non-[0,1] in a bool field) and then the kernel
        // ends up committing UB.  Maybe all SyscallArgs fields need to be MaybeUninit and each Syscall needs to validate all enum values, bool
        // values, etc. ?  Probably the most sound approach and perhaps the only approach, but this (correctly) puts the onus on any Syscall
        // implementers - we just need to provide adequate signposting and explicitness when passing around 'stuff' that originated in userspace.

        // The Cortex M4 crate can surround this with a #[cfg(not(feature = "no_default_syscall_invocation"))] to allow each MCU to define their own
        // implementation if there is something special that needs to be taken into account.

        // Keep the implementation as small as possible with pretty much no error checking.  Syscalls will be invoked many times (many call-sites) and the
        // fastest and smallest implementation will be perhaps three or four inlined assembly instructions, which will be comparable to an actual function
        // call sequence and also avoids stack frame overhead when not inlining.  Note that the onus for error checking is on the handler that runs in
        // privileged mode, so any checking here is just superfluous bloat for the inlined calls.

        println!("Invoking syscall {:x}", T::syscall_id());
        let result = unsafe { syscall_isr(T::syscall_id(), self as *mut Self as usize) };
        if result == 0 {
            println!("Syscall {:x} invoked successfully !", T::syscall_id());
            Ok(())
        } else {
            println!("OOPS !  Syscall {:x} returned error {} !", T::syscall_id(), result);
            Err(result)
        }
*/
        compiler_fence(Ordering::Release);

        let id = Self::syscall_id();
        let self_ptr = &raw mut *self;
        let mut result: usize;

        cfg_if::cfg_if! {
            if #[cfg(target_arch = "arm")] {
                use core::arch::asm;
                unsafe {
                    asm!(
                        "svc #0x00",
                        in("r0") id,
                        inout("r1") self_ptr => result,
                        options(preserves_flags));
                }
            } else {
                panic!("Cannot invoke a Cortex M4 Syscall on a non-Cortex M4 (running tests ?)");
            }
        }

        compiler_fence(Ordering::Acquire);

        unsafe { UsizeResult::from_usize_unchecked(result).as_result_unchecked() }
    }
}
