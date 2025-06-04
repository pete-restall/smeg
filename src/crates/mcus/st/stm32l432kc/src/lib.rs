#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod bootstrapping;

#[cfg(target_arch = "arm")]
mod reset_handler;

#[cfg(target_arch = "arm")]
mod kernel_stack;

#[cfg(target_arch = "arm")]
pub mod interrupts;

#[cfg(target_arch = "arm")]
mod blinky_blinky;
