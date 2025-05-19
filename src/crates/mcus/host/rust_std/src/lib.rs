#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod bootstrapping;

mod mcu_core;
pub(crate) use mcu_core::*;

// TODO: Eventually when proper symbols are used, this ought to be able to be deleted...
pub fn _needed_to_prevent_linker_gc() {
    panic!("Should never be called, since the only purpose is to prevent the linker from optimising away symbols it thinks are not used");
}
