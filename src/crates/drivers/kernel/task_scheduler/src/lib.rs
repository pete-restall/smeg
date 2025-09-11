#![cfg_attr(not(any(test, feature = "std")), no_std)]

#![feature(naked_functions)]

#![doc = smeg_kernel::docs::side_by_side_md!()]
use smeg_kernel::docs;

use core::convert::AsMut;

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
    dependencies: D
}

impl<D: Dependencies> Driver<D> {
    pub const fn new(dependencies: D) -> Self {
        Self { dependencies }
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
pub use yield_syscall::*;

smeg_kernel::def_private_api_token!(Driver);

#[macro_export]
macro_rules! import_driver {
    ($deps:ident $($args:tt)?) => {
        const {
            type Driver = ::smeg_drivers_kernel_task_scheduler::Driver<$deps>;
            const DRIVER: Driver = Driver::new($deps $($args)?);

            ::smeg_drivers_kernel_syscall::syscall_map! {
                YieldSyscall -> ::smeg_drivers_kernel_task_scheduler::YieldSyscallHandler<$deps>
            }

            DRIVER
        }
    };
}
