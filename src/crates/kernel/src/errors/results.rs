use core::num::NonZero;

use crate::HalfUsize;
use crate::docs;
use crate::errors::{ErrorTag, KernelErrorCode, TaggedError};

#[doc = docs::side_by_side_md!("KernelError")]
pub type KernelError = TaggedError<KernelErrorCode>;

#[doc = docs::side_by_side_md!("Result")]
pub type Result<T> = core::result::Result<T, KernelError>;

#[doc = docs::side_by_side_md!("ResultToUsizeResultConversion")]
pub trait ResultToUsizeResultConversion {
    #[doc = docs::side_by_side_md!("ResultToUsizeResultConversion.as_usize_result")]
    fn as_usize_result(self) -> UsizeResult;
}

impl ResultToUsizeResultConversion for Result<()> {
    #[doc = docs::side_by_side_md!("Result.as_usize_result")]
    fn as_usize_result(self) -> UsizeResult {
        match self {
            Ok(ok) => UsizeResult::Ok(ok),
            Err(err) => UsizeResult::Err(UsizeKernelError { value: NonZero::<usize>::from(err) })
        }
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
#[doc = docs::side_by_side_md!("UsizeKernelError")]
pub struct UsizeKernelError {
    value: NonZero<usize>
}

#[doc = docs::side_by_side_md!("UsizeResult")]
pub type UsizeResult = core::result::Result<(), UsizeKernelError>;

const _: () = {
    assert!(size_of::<UsizeResult>() == size_of::<usize>(), "Size of UsizeResult must be exactly one machine word");
    assert!(align_of::<UsizeResult>() == align_of::<usize>(), "Alignment of UsizeResult must be the same as a machine word");
};

#[doc = docs::side_by_side_md!("UsizeResultConversions")]
pub unsafe trait UsizeResultConversions {
    #[doc = docs::side_by_side_md!("UsizeResultConversions.from_usize_unchecked")]
    unsafe fn from_usize_unchecked(value: usize) -> Self;

    #[doc = docs::side_by_side_md!("UsizeResultConversions.as_result_unchecked")]
    unsafe fn as_result_unchecked(self) -> Result<()>;

    #[doc = docs::side_by_side_md!("UsizeResultConversions.as_usize")]
    fn as_usize(&self) -> usize;
}

unsafe impl UsizeResultConversions for UsizeResult {
    #[doc = docs::side_by_side_md!("UsizeResult.from_usize_unchecked")]
    unsafe fn from_usize_unchecked(value: usize) -> Self {
        unsafe { *(&value as *const usize as *const UsizeResult) }
    }

    #[doc = docs::side_by_side_md!("UsizeResult.as_result_unchecked")]
    unsafe fn as_result_unchecked(self) -> Result<()> {
        const {
            assert!(size_of::<KernelErrorCode>() == size_of::<u16>(), "Size of KernelErrorCode must be exactly 16 bits");
            assert!(size_of::<ErrorTag>() == size_of::<HalfUsize>(), "Size of ErrorTag must be half of a machine word");
        }

        match self {
            Ok(ok) => Ok(ok),
            Err(err) => unsafe {
                let err = err.value.get();
                let (code, tag) = ((err >> HalfUsize::BITS) as HalfUsize, err as HalfUsize);
                Err(TaggedError::new(
                    *(&(code as u16).to_be_bytes() as *const [u8; 2] as *const KernelErrorCode),
                    *(&tag as *const HalfUsize as *const ErrorTag)
                ))
            }
        }
    }

    #[doc = docs::side_by_side_md!("UsizeResult.as_usize")]
    fn as_usize(&self) -> usize {
        unsafe { *(self as *const UsizeResult as *const usize) }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use crate::errors::ErrorTag;

    use crate::test_doubles::Stub;

    use super::*;

    mod result {
        use fluent_test::prelude::*;

        use super::super::*;
        use super::*;

        #[test]
        fn as_usize_result__called_with_ok__expect_ok() {
            let ok: Result<()> = Ok(());
            let ok_as_usize = ok.as_usize_result();
            expect!(ok_as_usize).to_be_ok();
        }

        #[test]
        fn as_usize_result__called_with_err__expect_err_with_same_usize_value() {
            for error in sample_of_all_kernel_errors() {
                let err: Result<()> = Err(error);
                let err_as_usize = err.as_usize_result();
                let error_as_usize = NonZero::<usize>::from(error);
                expect!(err_as_usize.unwrap_err().value).to_equal(error_as_usize);
            }
        }
    }

    fn sample_of_all_kernel_errors() -> [KernelError; 11] {
        use crate::errors::kernel_error_code::test_doubles::sample_of_all_kernel_error_codes;
        sample_of_all_kernel_error_codes().map(|code| KernelError::new(code, ErrorTag::from(Stub)))
    }

    mod usize_result {
        use fluent_test::prelude::*;

        use smeg_testing_host_utils::integers::any_usize_except;

        use super::super::*;
        use super::*;

        #[test]
        fn from_usize_unchecked__called_with_zero_usize__expect_ok() {
            let usize_as_result = unsafe { UsizeResult::from_usize_unchecked(0) };
            expect!(usize_as_result).to_be_ok();
        }

        #[test]
        fn from_usize_unchecked__called_with_nonzero_usize__expect_err() {
            let usize_as_result = unsafe { UsizeResult::from_usize_unchecked(any_usize_except(0)) };
            expect!(usize_as_result).to_be_err();
        }

        #[test]
        fn as_result_unchecked__called_with_ok__expect_ok() {
            let ok = UsizeResult::Ok(());
            let result = unsafe { ok.as_result_unchecked() };
            expect!(result).to_be_ok();
        }

        #[test]
        fn as_result_unchecked__called_with_valid_err__expect_err_with_same_value() {
            for error in sample_of_all_kernel_errors() {
                let error_as_usize = NonZero::<usize>::from(error);
                let err = UsizeResult::Err(UsizeKernelError { value: error_as_usize });
                let result = unsafe { err.as_result_unchecked() };
                expect!(NonZero::<usize>::from(result.unwrap_err())).to_equal(error_as_usize);
            }
        }

        #[test]
        fn as_usize__called_with_ok__expect_zero() {
            let ok = UsizeResult::Ok(());
            expect!(ok.as_usize()).to_equal(0_usize);
        }

        #[test]
        fn as_usize__called_with_valid_err__expect_correct_usize() {
            for error in sample_of_all_kernel_errors() {
                let error_as_usize = NonZero::<usize>::from(error);
                let err = UsizeResult::Err(UsizeKernelError { value: error_as_usize });
                expect!(err.as_usize()).to_equal(error_as_usize.get());
            }
        }
    }
}

#[allow(unused)]
#[cfg(any(test, feature = "test_doubles"))]
pub mod test_doubles {
    use smeg_kernel_procmacro::error_tag;

    use crate::errors::test_doubles::any_kernel_error_code;

    use super::KernelError;

    pub fn any_kernel_error() -> KernelError {
        KernelError::new(any_kernel_error_code(), error_tag!("stubbed with any_kernel_error()"))
    }
}

pub mod prelude {
    pub use super::{KernelError, Result, ResultToUsizeResultConversion};
    pub use super::{UsizeKernelError, UsizeResult, UsizeResultConversions};
}
