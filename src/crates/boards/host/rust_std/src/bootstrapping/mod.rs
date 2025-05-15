pub use smeg_mcu_host_rust_std::bootstrapping::*;

pub mod kernel;

#[cfg(not(test))]
pub use smeg_mcu_host_rust_std::bootstrapping::entrypoint;
