#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use crate::HalfUsize;

// TODO: #[derive(Copy, Clone, Debug)]
#[doc = docs::side_by_side_md!("SyscallErrorCode")]
#[cfg_attr(target_pointer_width = "32", repr(C, u16))]
#[cfg_attr(target_pointer_width = "64", repr(C, u32))]
pub enum SyscallErrorCode {
    UnknownSyscall = 1,
    DriverSpecificErrorCode(HalfUsize)
}

#[doc = docs::side_by_side_md!("SyscallResult")]
pub type SyscallResult = Result<(), SyscallErrorCode>;

const _: () = {
    assert!(size_of::<SyscallResult>() == size_of::<usize>(), "Size of SyscallResult must be exactly one machine word");
};
