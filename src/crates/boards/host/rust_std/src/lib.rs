#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod bootstrapping;

// TODO: Eventually when proper symbols are used, this ought to be able to be deleted...
pub fn _needed_to_prevent_linker_gc() {
    smeg_mcu_host_rust_std::_needed_to_prevent_linker_gc();
}
