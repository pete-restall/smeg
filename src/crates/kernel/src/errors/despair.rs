use super::KernelError;

#[macro_export]
macro_rules! despair {
    (with($with:expr), because($($because:literal),+)) => {
        use ::smeg_kernel::errors::{error_tag, KernelError, TaggedError};
        unsafe extern "Rust" { #[unsafe(no_mangle)] unsafe fn __smeg_is_in_despair(squid: KernelError) -> !; }
        unsafe { __smeg_is_in_despair(TaggedError::new($with, error_tag!($($because),+))); }
    };

    (with($with:expr)) => {
        use ::smeg_kernel::errors::{error_tag, KernelError, TaggedError};
        unsafe extern "Rust" { #[unsafe(no_mangle)] unsafe fn __smeg_is_in_despair(squid: KernelError) -> !; }
        unsafe { __smeg_is_in_despair(TaggedError::new($with, error_tag!("In despair because of ", stringify!($with)))); }
    };

    (because($($because:literal),+)) => {
        use ::smeg_kernel::errors::{error_tag, KernelError, KernelErrorCode, TaggedError};
        unsafe extern "Rust" { #[unsafe(no_mangle)] unsafe fn __smeg_is_in_despair(squid: KernelError) -> !; }
        unsafe { __smeg_is_in_despair(TaggedError::new(KernelErrorCode::GeneralDespair, error_tag!($($because),+))); }
    };
}

#[unsafe(no_mangle)]
#[cfg(not(feature = "no_default_despair"))]
unsafe extern "Rust" fn __smeg_is_in_despair(squid: KernelError) -> ! {
    let mut squid_as_usize = 0;
    unsafe { core::ptr::write_volatile(&mut squid_as_usize, usize::from(squid)); }
    loop { /* Attach a debugger to examine the error code and tag on the stack */ }
}
