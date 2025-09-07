#![cfg_attr(not(any(test, feature = "std")), no_std)]

use smeg_kernel::McuSingleCore;
use smeg_kernel::interrupts::{HasFamilyIsrContext, HasIsrContext};

pub mod bootstrapping;

#[cfg(target_arch = "arm")]
mod reset_handler;

pub mod interrupts;

#[cfg(target_arch = "arm")]
mod isr_stack;

pub struct Driver;

impl Driver {
    pub const fn new() -> Self {
        Self { }
    }
}

impl McuSingleCore for Driver { }

impl HasIsrContext for Driver {
    type IsrContext = interrupts::IsrContext;
}

impl HasFamilyIsrContext for Driver {
    type FamilyIsrContext = interrupts::FamilyIsrContext;
}
