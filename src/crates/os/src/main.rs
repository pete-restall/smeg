#![cfg_attr(not(test), no_std)]
#![no_main]
//#![cfg_attr(not(feature = "smeg-board-host-native"), no_main)]

#[cfg(feature = "smeg-board-host-native")]
pub use smeg_board_host_native::main;

#[allow(unused_imports)]
use smeg_kernel;

// TODO: Eventually when proper symbols are used, this ought to be able to be deleted...
#[cfg(feature = "smeg-board-st-nucleo_l432kc-default")]
pub fn needed_to_prevent_linker_gc() {
	extern crate smeg_board_st_nucleo_l432kc;
	smeg_board_st_nucleo_l432kc::needed_to_prevent_linker_gc();
}
