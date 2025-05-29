#![cfg_attr(not(any(test, feature = "std")), no_std, no_main)]

pub mod board;
pub mod kernel;
pub mod rust;

#[cfg(all(not(test), feature = "smeg-board-host-rust_std"))]
#[unsafe(no_mangle)]
pub fn main() -> isize { board::bootstrapping::entrypoint().unwrap() }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __smeg_os_entrypoint() -> ! {
    use smeg_kernel::bootstrapping::kernel::Entrypoint;
    use kernel::Kernel;
    unsafe { Kernel::entrypoint() }
}
