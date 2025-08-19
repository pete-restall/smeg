use smeg_kernel::interrupts::IsrContext;

use crate::{SyscallArgs, SyscallResult};

use super::SyscallIsrContext;

pub trait SyscallIsrHandler {
    type IsrContext: IsrContext;
    type Args: SyscallArgs;

    fn on_syscall(context: &mut SyscallIsrContext<Self::IsrContext, Self::Args>) -> SyscallResult;
}
