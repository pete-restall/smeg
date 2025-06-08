// TODO: Unsound implementation - should've read the Result and Option docs better.  Only guaranteed layout, including None == 0, for certain references,
// built-ins and NonZero<Primitive>s  Rethink the implementation, but not the approach...  A better idea is also to leverage the KernelErrorCodes.

#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use crate::HalfUsize;

#[derive(Copy, Clone, Debug, PartialEq)]
#[doc = docs::side_by_side_md!("SyscallErrorCode")]
#[cfg_attr(target_pointer_width = "32", repr(C, u16))]
#[cfg_attr(target_pointer_width = "64", repr(C, u32))]
pub enum SyscallErrorCode {
    UnknownSyscall = 1,
    DriverSpecificErrorCode(HalfUsize)
}

#[doc = docs::side_by_side_md!("SyscallResult")]
pub type SyscallResult = Result<(), SyscallErrorCode>;

const _: () = {
    assert!(size_of::<SyscallResult>() == size_of::<usize>(), "Size of SyscallResult must be exactly one machine word");
};

#[doc = docs::side_by_side_md!("SyscallResultUsizeConversion")]
pub trait SyscallResultUsizeConversion {
    #[doc = docs::side_by_side_md!("SyscallResultUsizeConversion.into_usize")]
    fn into_usize(self) -> usize;

    #[doc = docs::side_by_side_md!("SyscallResultUsizeConversion.from_usize_unchecked")]
    unsafe fn from_usize_unchecked(value: usize) -> Self;
}

impl SyscallResultUsizeConversion for SyscallResult {
    #[doc = docs::side_by_side_md!("SyscallResult.into_usize")]
    fn into_usize(self) -> usize {
        unsafe { *(&self as *const SyscallResult as *const usize) }
    }

    #[doc = docs::side_by_side_md!("SyscallResult.from_usize_unchecked")]
    unsafe fn from_usize_unchecked(value: usize) -> Self {
        unsafe { *(&value as *const usize as *const SyscallResult) }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use smeg_testing_host_utils::integers::any_usize;

    use super::*;

    mod syscall_error_code {
        use fluent_test::prelude::*;

        use super::*;

        #[test]
        fn discriminant__get_for_each__expect_all_nonzero() {
            for error_code in all_syscall_error_codes_with_zero_for_all_args() {
                expect!(discriminant_of(error_code)).not().to_equal(0);
            }
        }

        #[test]
        fn equality__tested_after_copy__expect_same_value() {
            let original_error_codes = all_syscall_error_codes_with_any_args();
            let copied_error_codes = original_error_codes;
            expect!(&copied_error_codes).to_equal_collection(&original_error_codes);
        }
    }

    fn all_syscall_error_codes_with_zero_for_all_args() -> [SyscallErrorCode; 2] {
        all_syscall_error_codes(false)
    }

    fn all_syscall_error_codes(use_any_arg: bool) -> [SyscallErrorCode; 2] {
        [
            SyscallErrorCode::UnknownSyscall,
            SyscallErrorCode::DriverSpecificErrorCode(if use_any_arg { any_half_usize() } else { 0 })
        ]
    }

    fn any_half_usize() -> HalfUsize {
        any_usize() as HalfUsize
    }

    fn all_syscall_error_codes_with_any_args() -> [SyscallErrorCode; 2] {
        all_syscall_error_codes(true)
    }

    fn discriminant_of(error_code: SyscallErrorCode) -> HalfUsize {
        unsafe { *(&error_code as *const SyscallErrorCode as *const HalfUsize) }
    }

    mod syscall_result {
        use fluent_test::prelude::*;

        use smeg_testing_host_utils::integers::any_usize_except;

        use super::super::*;
        use super::*;

        #[test]
        fn equality__tested_after_clone__expect_same_value() {
            let original_error_codes = all_syscall_error_codes_with_any_args();
            let cloned_error_codes = original_error_codes.clone();
            expect!(&cloned_error_codes).to_equal_collection(&original_error_codes);
        }

        #[test]
        fn into_usize__called_with_ok__expect_zero() {
            let ok: SyscallResult = Ok(());
            let ok_as_usize = ok.into_usize();
            expect!(ok_as_usize).to_equal(0);
        }

        #[test]
        fn into_usize__called_with_err__expect_nonzero() {
            for error_code in all_syscall_error_codes_with_any_args() {
                let error: SyscallResult = Err(error_code);
                let err_as_usize = error.into_usize();
                expect!(err_as_usize).not().to_equal(0);
            }
        }

        #[test]
        fn from_usize_unchecked__called_with_zero_usize__expect_ok() {
            let usize_as_result = unsafe { SyscallResult::from_usize_unchecked(0) };
            expect!(usize_as_result).to_be_ok();
        }

        #[test]
        fn from_usize_unchecked__called_with_nonzero_usize__expect_err() {
            let usize_as_result = unsafe { SyscallResult::from_usize_unchecked(any_usize_except(0)) };
            expect!(usize_as_result).to_be_err();
        }

        #[test]
        fn ok_instance__roundtripped_to_usize_and_back__expect_value_is_ok() {
            let ok: SyscallResult = Ok(());
            let ok_as_usize = ok.into_usize();
            let roundtripped_result = unsafe { SyscallResult::from_usize_unchecked(ok_as_usize) };
            expect!(roundtripped_result).to_be_ok();
        }

        #[test]
        fn err_instance__roundtripped_to_usize_and_back__expect_same_value() {
            for error_code in all_syscall_error_codes_with_any_args() {
                let err: SyscallResult = Err(error_code);
                let err_as_usize = err.into_usize();
                let roundtripped_result = unsafe { SyscallResult::from_usize_unchecked(err_as_usize) };
                expect!(roundtripped_result).to_equal(err);
            }
        }
    }
}
