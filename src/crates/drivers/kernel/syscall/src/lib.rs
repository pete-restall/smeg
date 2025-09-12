#![cfg_attr(not(any(test, feature = "std")), no_std)]

#![feature(naked_functions)]

use core::convert::AsMut;
use core::marker::PhantomData;

use smeg_kernel::docs;
use smeg_kernel::interrupts::{HasIsrContext, IsrContext, NoIsrContext};

pub mod isr;
use isr::SyscallIsrTrampolinePtr;

#[path = "mcu/mod.rs"]
mod _mcu;

cfg_if::cfg_if! {
    if #[cfg(not(any(test, feature = "test_doubles")))] {
        use _mcu as mcu;
    } else {
        use _mcu::test_doubles as mcu;
    }
}

mod syscall_args;
pub use syscall_args::*;

mod syscall_map;

pub use smeg_drivers_kernel_syscall_procmacro::syscall_args;

#[doc = docs::side_by_side_md!("SyscallResult")]
pub type SyscallResult = smeg_kernel::errors::Result<()>;

pub trait SyscallInvocation {
    fn invoke_syscall(&mut self) -> SyscallResult;
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
    type IsrContext = NoIsrContext; // TODO: this will be the API we wish to export to the ISR context for other drivers to use
}

pub trait Dependencies {
    type IsrContext: IsrContext + From<mcu::IsrContext> + AsMut<mcu::IsrContext>;

    fn trampoline_vector_table<'a>() -> Option<&'a [SyscallIsrTrampolinePtr<Self::IsrContext>]> {
        assert!(
            size_of::<SyscallIsrTrampolinePtr<Self::IsrContext>>() == size_of::<SyscallIsrTrampolinePtr<usize>>(),
            "This code makes the assumption that a Syscall trampoline pointer is the same size regardless of its argument");

        unsafe extern "Rust" {
            static __LINKER_DRIVERS_SYSCALL_ISR_TRAMPOLINES_START: SyscallIsrTrampolinePtr<usize>;
            static __LINKER_DRIVERS_SYSCALL_ISR_TRAMPOLINES_PAST_END: SyscallIsrTrampolinePtr<usize>;
        }

        unsafe {
            smeg_kernel::try_slice_from(
                &raw const __LINKER_DRIVERS_SYSCALL_ISR_TRAMPOLINES_START as *const SyscallIsrTrampolinePtr<Self::IsrContext>,
                &raw const __LINKER_DRIVERS_SYSCALL_ISR_TRAMPOLINES_PAST_END as *const SyscallIsrTrampolinePtr<Self::IsrContext>)
        }
    }
}

#[macro_export]
macro_rules! import_driver {
    ($deps:ident $($args:tt)?) => {
        const {
            type Driver = ::smeg_drivers_kernel_syscall::Driver<$deps>;
            const DRIVER: Driver = Driver::new($deps $($args)?);
            DRIVER
        }
    };
}
