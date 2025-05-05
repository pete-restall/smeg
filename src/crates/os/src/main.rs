#![cfg_attr(not(any(test, feature = "std")), no_std, no_main)]

#[cfg(not(test))]
#[cfg(feature = "smeg-board-host-rust_std")]
pub use smeg_board_host_rust_std::main;

#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use smeg_kernel;

// TODO: Eventually when proper symbols are used, this ought to be able to be deleted...
#[cfg(feature = "smeg-board-st-nucleo_l432kc-default")]
pub fn needed_to_prevent_linker_gc() {
    extern crate smeg_board_st_nucleo_l432kc;
    smeg_board_st_nucleo_l432kc::needed_to_prevent_linker_gc();
}
