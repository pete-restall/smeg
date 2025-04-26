#![cfg_attr(not(any(test, feature = "std")), no_std)]

use smeg_config_procmacro::populate_from_config;

#[populate_from_config]
pub struct Config;
