#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod bootstrapping;

#[cfg(all(not(test), feature = "std"))]
pub mod panic_handler;

#[cfg(not(all(not(test), feature = "std")))]
mod panic_handler;

pub trait HasMcuCoreId {
    fn core_id() -> usize;
}
