#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod interrupts;
pub mod mem;
pub mod ppb;

pub use smeg_mcu_arm_cortex_m4_family_procmacro::arm_register;
