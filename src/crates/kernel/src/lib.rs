#![cfg_attr(not(any(test, feature = "std")), no_std)]

pub mod bootstrapping;

pub mod docs;

pub mod errors;

#[cfg(target_pointer_width = "32")]
pub type HalfUsize = u16;

#[cfg(target_pointer_width = "64")]
pub type HalfUsize = u32;

#[cfg(all(not(test), feature = "std"))]
pub mod panic_handler;

#[cfg(not(all(not(test), feature = "std")))]
mod panic_handler;

pub(crate) extern crate self as smeg_kernel;

#[cfg(test)]
pub mod test_doubles;

// TODO: Below here needs moving somewhere sensible, once it becomes more obvious where...
pub trait HasMcuCoreId {
    fn core_id() -> usize;
}

pub struct McuSingleCore;

impl HasMcuCoreId for McuSingleCore {
    fn core_id() -> usize { 0 }
}

pub fn is_rust_runtime_initialised() -> bool {
    use core::mem::MaybeUninit;
    #[unsafe(link_section = ".data.flags.guaranteed_zero_on_reset.0")]
    static IS_RUST_RUNTIME_INITIALISED: MaybeUninit<bool> = MaybeUninit::new(true);
    unsafe { core::ptr::read_volatile(IS_RUST_RUNTIME_INITIALISED.as_ptr()) }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use super::*;

    #[test]
    fn core_id__called__expect_hard_coded_zero() {
        expect!(McuSingleCore::core_id()).to_equal(0);
    }
}
