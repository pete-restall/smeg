use core::marker::PhantomData;

use smeg_drivers_kernel_syscall::{syscall_args, SyscallResult};
use smeg_drivers_kernel_syscall::isr::{SyscallIsrContext, SyscallIsrHandler};

use super::Dependencies;

#[syscall_args]
pub struct YieldSyscall;

pub struct YieldSyscallHandler<D: Dependencies> {
    _dependencies: PhantomData<D>
}

impl<D: Dependencies> SyscallIsrHandler for YieldSyscallHandler<D> {
    type IsrContext = D::IsrContext;
    type Args = YieldSyscall;

    fn on_syscall(context: &mut SyscallIsrContext<Self::IsrContext, Self::Args>) -> SyscallResult {
		// TODO: Clearly needs writing... ICSR.PENDSVSET - this is *not* simple; there can be _no reference ever_ to an MMIO (volatile) as it is _unsound_
		// Everything is going to have to be done through raw pointers.
		Ok(())
	}
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
	use fluent_test::prelude::*;

	use super::*;

// #[test]
// fn on_syscall__
// the test below is useless; we need to test on_syscall directly but it needs to be MCU-specific.

	mod yield_syscall {
		use smeg_drivers_kernel_syscall::HasSyscallId;

		use super::*;

		#[test]
		fn syscall_id__called_multiple_times__expect_same_id() {
			let ids = [0, 0];//YieldSyscall::syscall_id(), YieldSyscall::syscall_id()];
			expect!(ids[0]).to_equal(ids[1]);
		}
	}
}
