#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod interrupts;

#[cfg(target_arch = "arm")]
pub mod syscalls;
