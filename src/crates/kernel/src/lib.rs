#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(all(not(test), feature = "std"))]
pub mod panic_handler;

#[cfg(not(all(not(test), feature = "std")))]
mod panic_handler;

#[allow(unused_imports)] // TODO: temporary, just to keep the artefacts...
use smeg_config::Config;
