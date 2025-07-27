#![cfg_attr(not(any(test, feature = "std")), no_std)]

#![feature(naked_functions)]

use core::marker::PhantomData;

use smeg_kernel::syscalls::SyscallResult;

pub mod isr;
use isr::SyscallIsrTrampolinePtr;

mod mcu;
use mcu::{collect_isr_vectors, IsrContext, IsrVectorTableBuilder};

mod syscall_args;
pub use syscall_args::*;

mod syscall_map;

pub use smeg_drivers_kernel_syscall_procmacro::syscall_args;

pub trait SyscallInvocation {
    fn invoke_syscall(&mut self) -> SyscallResult;
}

pub struct Driver<D: Dependencies> {
    _dependencies: PhantomData<D>
}

impl<D: Dependencies> Driver<D> {
    pub const fn collect_isr_vectors(isrs: IsrVectorTableBuilder) -> IsrVectorTableBuilder {
        collect_isr_vectors::<D>(isrs)
    }
}

pub trait Dependencies {
    type IsrContext: IsrContext;

    fn trampoline_vector_table<'a>() -> Option<&'a [SyscallIsrTrampolinePtr<Self::IsrContext>]> {
        assert!(
            size_of::<SyscallIsrTrampolinePtr<Self::IsrContext>>() == size_of::<SyscallIsrTrampolinePtr<usize>>(),
            "This code makes the assumption that a Syscall trampoline pointer is the same size regardless of its argument");

        unsafe extern "Rust" {
            static __LINKER_SYSCALLS_ISR_TRAMPOLINES_START: SyscallIsrTrampolinePtr<usize>;
            static __LINKER_SYSCALLS_ISR_TRAMPOLINES_PAST_END: SyscallIsrTrampolinePtr<usize>;
        }

        unsafe {
            smeg_kernel::try_slice_from(
                &raw const __LINKER_SYSCALLS_ISR_TRAMPOLINES_START as *const SyscallIsrTrampolinePtr<Self::IsrContext>,
                &raw const __LINKER_SYSCALLS_ISR_TRAMPOLINES_PAST_END as *const SyscallIsrTrampolinePtr<Self::IsrContext>)
        }
    }
}
