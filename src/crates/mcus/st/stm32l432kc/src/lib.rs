#![cfg_attr(not(any(test, feature = "std")), no_std)]

use smeg_kernel::McuSingleCore;
use smeg_kernel::interrupts::{HasFamilyIsrContext, HasIsrContext};

pub mod bootstrapping;

#[cfg(target_arch = "arm")]
mod reset_handler;

pub mod interrupts;

#[cfg(target_arch = "arm")]
mod isr_stack;

pub trait Dependencies { }

pub struct Driver<D: Dependencies> {
    dependencies: D
}

impl<D: Dependencies> Driver<D> {
    pub const fn new(dependencies: D) -> Self {
        Self { dependencies }
    }
}

impl<D: Dependencies> McuSingleCore for Driver<D> { }

impl<D: Dependencies> HasIsrContext for Driver<D> {
    type IsrContext = interrupts::IsrContext;
}

impl<D: Dependencies> HasFamilyIsrContext for Driver<D> {
    type FamilyIsrContext = interrupts::FamilyIsrContext;
}

#[macro_export]
macro_rules! import_driver {
    ($deps:ident $($args:tt)?) => {
        const {
            type Driver = ::smeg_mcu_st_stm32l432kc::Driver<$deps>;
            const DRIVER: Driver = Driver::new($deps $($args)?);
            DRIVER
        }
    };
}
