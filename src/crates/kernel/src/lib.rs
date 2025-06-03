#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(feature = "no_default_despair", feature(linkage))]

pub mod bootstrapping;

pub mod docs;

pub mod errors;

mod has_mcu_core_id;
pub use has_mcu_core_id::*;

#[cfg(target_pointer_width = "32")]
pub type HalfUsize = u16;

#[cfg(target_pointer_width = "64")]
pub type HalfUsize = u32;

#[cfg(all(not(test), feature = "std"))]
pub mod panic_handler;

#[cfg(not(all(not(test), feature = "std")))]
mod panic_handler;

pub(crate) extern crate self as smeg_kernel;

pub(crate) mod caller {
    pub enum IsKernel { }
    pub trait RestrictedToKernel { }
    impl RestrictedToKernel for IsKernel { }
}

#[cfg(test)]
pub mod test_doubles;
pub fn is_rust_runtime_initialised() -> bool {
    use core::mem::MaybeUninit;
    #[unsafe(link_section = ".data.flags.guaranteed_zero_on_reset.0")]
    static IS_RUST_RUNTIME_INITIALISED: MaybeUninit<bool> = MaybeUninit::new(true);
    unsafe { core::ptr::read_volatile(IS_RUST_RUNTIME_INITIALISED.as_ptr()) }
}
