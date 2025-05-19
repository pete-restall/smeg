#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod bootstrapping;

#[cfg(all(not(test), feature = "std"))]
pub mod panic_handler;

#[cfg(not(all(not(test), feature = "std")))]
mod panic_handler;

pub mod errors;

#[cfg(target_pointer_width = "32")]
pub type HalfUsize = u16;

#[cfg(target_pointer_width = "64")]
pub type HalfUsize = u32;

// TODO: This needs moving somewhere sensible
pub trait HasMcuCoreId {
    fn core_id() -> usize;
}
