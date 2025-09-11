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
		// TODO: Clearly needs writing...
		Ok(())
	}
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
	use fluent_test::prelude::*;

	use super::*;

	mod yield_syscall {
	    use smeg_drivers_kernel_syscall::HasSyscallId;

    	use super::*;

		#[test]
		fn syscall_id__called_multiple_times__expect_same_id() {
			let ids = [YieldSyscall::syscall_id(), YieldSyscall::syscall_id()];
			expect!(ids[0]).to_equal(ids[1]);
		}
	}
}
