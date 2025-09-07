#![cfg_attr(not(any(test, feature = "std")), no_std)]

#![feature(naked_functions)]

#![doc = smeg_kernel::docs::side_by_side_md!()]
use smeg_kernel::docs;

use core::convert::AsMut;
use core::marker::PhantomData;

use smeg_kernel::interrupts::{HasIsrContext, IsrContext, NoIsrContext};

#[path = "mcu/mod.rs"]
mod _mcu;

cfg_if::cfg_if! {
    if #[cfg(not(any(test, feature = "test_doubles")))] {
        use _mcu as mcu;
    } else {
        use _mcu::test_doubles as mcu;
    }
}

pub struct Driver<D: Dependencies> {
    _dependencies: PhantomData<D>
}

impl<D: Dependencies> Driver<D> {
    pub const fn new() -> Self {
        Self { _dependencies: PhantomData }
    }

    pub const fn collect_isr_vectors(isrs: mcu::IsrVectorTableBuilder) -> mcu::IsrVectorTableBuilder {
        mcu::collect_isr_vectors::<D>(isrs)
    }
}

impl<D: Dependencies> HasIsrContext for Driver<D> {
    type IsrContext = NoIsrContext; // TODO: this will be the API we wish to export to the ISR context for other drivers to use - obviously we do, for scheduling
}

pub trait Dependencies {
    type IsrContext: IsrContext + From<mcu::IsrContext> + AsMut<mcu::IsrContext>;
}

mod yield_syscall;
