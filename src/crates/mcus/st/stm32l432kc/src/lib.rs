#![cfg_attr(not(test), no_std)]

mod reset_handler;
mod interrupts;

mod blinky_blinky;

// TODO: Eventually when proper symbols are used, this ought to be able to be deleted...
pub fn needed_to_prevent_linker_gc() {
    extern crate smeg_mcu_arm_cortex_m4;
    smeg_mcu_arm_cortex_m4::needed_to_prevent_linker_gc();
}
