#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

#[doc = docs::side_by_side_md!("McuSyscallInvocation")] // TODO: this will die in favour of the SyscallInvocation inside the Syscall driver
pub trait McuSyscallInvocation {
    #[doc = docs::side_by_side_md!("McuSyscallInvocation.invoke_syscall")]
    fn invoke_syscall(id: u8) -> SyscallResult;
}

#[doc = docs::side_by_side_md!("SyscallResult")] // I think this one can probably be moved into the syscall driver, too
pub type SyscallResult = smeg_kernel::errors::Result<()>;
