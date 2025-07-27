#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(target_arch = "arm")]
pub(crate) mod blinky_blinky; // TODO: can be removed once there are better ways to verify the firmware is up and running on the board...

pub mod bootstrapping;

pub(crate) mod drivers;
