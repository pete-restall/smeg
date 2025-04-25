#![cfg_attr(not(test), no_std)]

#[cfg(not(test))]
mod panic_handler;

#[allow(unused_imports)] // TODO: temporary, just to keep the artefacts...
use smeg_config::Config;
