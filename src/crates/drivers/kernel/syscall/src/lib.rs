#![cfg_attr(not(any(test, feature = "std")), no_std)]

#![feature(naked_functions)]

mod mcu;
pub use mcu::collect_isr_vectors;

pub struct Driver;
