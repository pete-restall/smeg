#![cfg_attr(not(any(test, feature = "std")), no_std)]

#![feature(naked_functions)]

use smeg_kernel::bootstrapping::kernel::IsrBootstrapping;

mod mcu;

pub struct Driver;

impl Driver {
    pub const fn collect_isr_vectors<I: IsrBootstrapping>(isrs: mcu::IsrVectorTableBuilder<I>) -> mcu::IsrVectorTableBuilder<I> {
        mcu::collect_isr_vectors(isrs)
    }
}
