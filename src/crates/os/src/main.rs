#![cfg_attr(not(any(test, feature = "std")), no_std, no_main)]

mod board;

#[cfg(all(not(test), feature = "smeg-board-host-rust_std"))]
#[unsafe(no_mangle)]
pub fn main() -> isize { board::bootstrapping::entrypoint().unwrap() }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __smeg_os_entrypoint() -> ! {
    use board::bootstrapping::{kernel, rust};
    unsafe {
        entrypoint::<rust::RuntimeBootstrapper, kernel::McuCoreBootstrapper, kernel::BoardMcuBootstrapper>();
    }
}

use smeg_kernel::bootstrapping::kernel::{BoardMcuBootstrapping, McuCoreBootstrapping};
use smeg_kernel::bootstrapping::rust::RuntimeBootstrapping;

unsafe fn entrypoint<R: RuntimeBootstrapping, C: McuCoreBootstrapping, B: BoardMcuBootstrapping>() -> ! {
    unsafe {
        if C::core_id() == 0 {
            smeg_kernel::bootstrapping::rust::initialise::<R>();
        }

        smeg_kernel::bootstrapping::kernel::entrypoint::<C, B>();
    }
}
