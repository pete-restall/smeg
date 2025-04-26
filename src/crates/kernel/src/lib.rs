#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(not(any(test, feature = "std")))]
mod panic_handler;

#[allow(unused_imports)] // TODO: temporary, just to keep the artefacts...
use smeg_config::Config;
