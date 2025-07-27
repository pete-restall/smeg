use smeg_kernel::interrupts::IsrContext;
use smeg_kernel::syscalls::SyscallResult;

use crate::SyscallArgs;

use super::SyscallIsrContext;

pub trait SyscallIsrHandler {
    type IsrContext: IsrContext;
    type Args: SyscallArgs;

    fn on_syscall(context: &mut SyscallIsrContext<Self::IsrContext, Self::Args>) -> SyscallResult;
}
