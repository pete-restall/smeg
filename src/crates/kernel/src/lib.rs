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

pub mod interrupts;

#[cfg(all(not(test), feature = "std"))]
pub mod panic_handler;

#[cfg(not(all(not(test), feature = "std")))]
mod panic_handler;

pub mod syscalls;

pub(crate) extern crate self as smeg_kernel;

pub(crate) mod caller {
    pub enum IsKernel { }
    pub trait RestrictedToKernel { }
    impl RestrictedToKernel for IsKernel { }
}

#[cfg(any(test, feature = "test_doubles"))]
pub mod test_doubles;

pub fn is_rust_runtime_initialised() -> bool {
    use core::mem::MaybeUninit;
    #[unsafe(link_section = ".data.flags.guaranteed_zero_on_reset.0")]
    static IS_RUST_RUNTIME_INITIALISED: MaybeUninit<bool> = MaybeUninit::new(true);
    unsafe { core::ptr::read_volatile(IS_RUST_RUNTIME_INITIALISED.as_ptr()) }
}

pub const fn const_unwrap_or<T: Copy>(maybe_none: Option<T>, alternative: T) -> T {
    // FIXME: This can be removed once Rust implements a const implementation for Option::unwrap_or(...)
    some_or(maybe_none, Some(alternative)).unwrap()
}

pub const fn some_or<T: Copy>(maybe_none: Option<T>, alternative: Option<T>) -> Option<T> {
    // FIXME: This can be removed once Rust implements a const implementation for Option::or(...)
    match maybe_none {
        None => alternative,
        some => some
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use fluent_test::prelude::*;

    use super::*;

    #[test]
    fn const_unwrap_or__called_with_none__expect_alternative() {
        const RESULT: bool = const {
            const NONE: Option<usize> = None;
            const ALTERNATIVE: usize = 123;
            const_unwrap_or(NONE, ALTERNATIVE) == ALTERNATIVE
        };

        expect!(RESULT).to_be_true();
    }

    #[test]
    fn const_unwrap_or__called_with_some__expect_same_value() {
        const RESULT: bool = const {
            const SOME: Option<char> = Some('x');
            const ALTERNATIVE: char = 'y';
            const_unwrap_or(SOME, ALTERNATIVE) == SOME.unwrap()
        };

        expect!(RESULT).to_be_true();
    }

    #[test]
    fn some_or__called_with_none__expect_alternative() {
        const RESULT: bool = const {
            const NONE: Option<u32> = None;
            const ALTERNATIVE: Option<u32> = Some(123);
            match some_or(NONE, ALTERNATIVE) {
                ALTERNATIVE => true,
                _ => false
            }
        };

        expect!(RESULT).to_equal(true);
    }

    #[test]
    fn some_or__called_with_some__expect_same_value() {
        const RESULT: bool = const {
            const SOME: Option<i16> = Some(456);
            const ALTERNATIVE: Option<i16> = Some(123);
            match some_or(SOME, ALTERNATIVE) {
                SOME => true,
                _ => false
            }
        };

        expect!(RESULT).to_equal(true);
    }

    #[test]
    fn some_or__called_with_none_and_no_alternative__expect_none() {
        const RESULT: bool = const {
            const NONE: Option<char> = None;
            const NO_ALTERNATIVE: Option<char> = None;
            some_or(NONE, NO_ALTERNATIVE).is_none()
        };

        expect!(RESULT).to_equal(true);
    }
}
