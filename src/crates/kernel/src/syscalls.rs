#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

#[doc = docs::side_by_side_md!("McuSyscallInvocation")]
pub trait McuSyscallInvocation {
    #[doc = docs::side_by_side_md!("McuSyscallInvocation.invoke_syscall")]
    fn invoke_syscall(id: u8) -> SyscallResult;
}

#[doc = docs::side_by_side_md!("SyscallResult")]
pub type SyscallResult = smeg_kernel::errors::Result<()>;
