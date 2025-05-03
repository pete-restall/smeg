#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(target_arch = "arm")]
mod reset_handler;

#[cfg(target_arch = "arm")]
mod interrupts;

#[cfg(target_arch = "arm")]
mod blinky_blinky;

// TODO: Eventually when proper symbols are used, this ought to be able to be deleted...
pub fn needed_to_prevent_linker_gc() {
    extern crate smeg_mcu_arm_cortex_m4_family;
    smeg_mcu_arm_cortex_m4_family::needed_to_prevent_linker_gc();
}
