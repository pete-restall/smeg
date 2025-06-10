#![doc = smeg_kernel::docs::side_by_side_md!()]
use smeg_kernel::docs;

use core::arch::asm;
use core::mem::MaybeUninit;
use core::sync::atomic::compiler_fence;

use smeg_kernel::errors::{UsizeResult, UsizeResultConversions};
use smeg_kernel::syscalls::{McuSyscallInvocation, SyscallResult};

#[doc = docs::side_by_side_md!("Syscalls")]
pub struct Syscalls;

impl McuSyscallInvocation for Syscalls {
    #[inline(always)]
    #[doc = docs::side_by_side_md!("Syscalls.invoke_syscall")]
    fn invoke_syscall(id: u8) -> SyscallResult {
        compiler_fence(core::sync::atomic::Ordering::Release);

        let mut result = MaybeUninit::uninit();
        unsafe {
            asm!(
                "svcall #0x00",
                in("r0") id as usize,
                out("r1") result,
                options(preserves_flags));
        }

        compiler_fence(core::sync::atomic::Ordering::Acquire);

        unsafe { UsizeResult::from_usize_unchecked(result.assume_init()).as_result_unchecked() }
    }
}
