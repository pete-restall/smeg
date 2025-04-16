#![cfg_attr(not(test), no_std)]

// TODO: Eventually when proper symbols are used, this ought to be able to be deleted...
pub fn needed_to_prevent_linker_gc() {
	extern crate smeg_mcu_st_stm32l432kc;
	smeg_mcu_st_stm32l432kc::needed_to_prevent_linker_gc();
}
