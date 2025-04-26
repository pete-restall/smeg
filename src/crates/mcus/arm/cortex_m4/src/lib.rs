#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod interrupts;

// TODO: Eventually when proper symbols are used, this ought to be able to be deleted...
pub fn needed_to_prevent_linker_gc() {
    panic!("Should never be called, since the only purpose is to prevent the linker from optimising away symbols it thinks are not used");
}
