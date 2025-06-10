#![cfg_attr(not(any(test, feature = "std")), no_std)]

#![feature(naked_functions)]

mod mcu;

pub struct Driver;

impl Driver {
    pub const fn collect_isr_vectors(isrs: mcu::IsrVectorTableBuilder) -> mcu::IsrVectorTableBuilder {
        mcu::collect_isr_vectors(isrs)
    }
}
