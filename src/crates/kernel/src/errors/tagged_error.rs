use core::mem::{align_of, size_of};
use core::num::NonZero;

use crate::HalfUsize;

use super::ErrorTag;

#[derive(Copy, Clone, Debug)]
#[cfg_attr(target_pointer_width = "32", repr(C, align(4)))]
#[cfg_attr(target_pointer_width = "64", repr(C, align(8)))]
pub struct TaggedError<T> where T : Copy + Clone + Into<NonZero<HalfUsize>> {
    pub code: T,
    pub tag: ErrorTag
}

impl<T> TaggedError<T> where T : Copy + Clone + Into<NonZero<HalfUsize>> {
    pub fn new(code: T, tag: ErrorTag) -> TaggedError<T> {
        const {
            assert!(size_of::<TaggedError<T>>() == size_of::<usize>(), "Size of TaggedError must be exactly one machine word");
            assert!(align_of::<TaggedError<T>>() == align_of::<usize>(), "Alignment of TaggedError must be the same as a machine word");
        }

        TaggedError::<T> { code, tag }
    }
}

impl<T> From<TaggedError<T>> for NonZero<usize> where T : Copy + Clone + Into<NonZero<HalfUsize>> {
    fn from(error: TaggedError<T>) -> Self {
        unsafe {
            NonZero::new_unchecked(
                (error.code.into().get() as usize) << HalfUsize::BITS |
                (HalfUsize::from(error.tag) as usize))
        }
    }
}

impl<T> From<&TaggedError<T>> for NonZero<usize> where T : Copy + Clone + Into<NonZero<HalfUsize>> {
    fn from(error: &TaggedError<T>) -> Self {
        (*error).into()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use core::fmt::Debug;
    use std::sync::OnceLock;

    use fluent_test::prelude::*;

    use smeg_kernel_procmacro::error_tag;
    use smeg_testing_host_utils::integers::{any_u8, any_usize_except};

    use super::*;

    #[repr(C, u8)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum StubEnumCode {
        A,
        B(u8),
        C
    }

    impl From<StubEnumCode> for NonZero<HalfUsize> {
        fn from(error: StubEnumCode) -> Self {
            static IDS: OnceLock<[NonZero<HalfUsize>; 3]> = OnceLock::new();
            let ids = IDS.get_or_init(|| [any_nonzero_half_usize(), any_nonzero_half_usize(), any_nonzero_half_usize()]);
            match error {
                StubEnumCode::A => ids[0],
                StubEnumCode::B(value) => {
                    let value = ids[1].get() ^ (value as HalfUsize);
                    NonZero::new(value).or(NonZero::new(1)).unwrap()
                },
                StubEnumCode::C => ids[2]
            }
        }
    }

    fn any_nonzero_half_usize() -> NonZero<HalfUsize> {
        NonZero::new(any_usize_except(0) as HalfUsize).unwrap()
    }

    #[test]
    fn code__get_after_new_when_integer__expect_same_value_passed_to_constructor() {
        code__get_after_new__expect_same_value_passed_to_constructor(any_integer_code());
    }

    fn code__get_after_new__expect_same_value_passed_to_constructor<T>(code: T)
        where T : Copy + Clone + Into<NonZero<HalfUsize>> + PartialEq + Debug {

        let error = TaggedError::new(code, dummy_tag());
        expect!(error.code).to_equal(code);
    }

    fn dummy_tag() -> ErrorTag {
        error_tag!("dummy error")
    }

    fn any_integer_code() -> NonZero<HalfUsize> {
        any_nonzero_half_usize()
    }

    #[test]
    fn code__get_after_new_when_enum__expect_same_value_passed_to_constructor() {
        for code in each_enum_scenario() {
            code__get_after_new__expect_same_value_passed_to_constructor(code);
        }
    }

    fn each_enum_scenario() -> [StubEnumCode; 3] {
        [StubEnumCode::A, StubEnumCode::B(any_u8()), StubEnumCode::C]
    }

    #[test]
    fn code__get_after_copied_when_integer__expect_same_value_as_original_error() {
        let original_error = TaggedError::new(any_integer_code(), dummy_tag());
        let copied_error = original_error;
        expect!(copied_error.code).to_equal(original_error.code);
    }

    #[test]
    fn code__get_after_cloned_when_integer__expect_same_value_as_original_error() {
        let original_error = TaggedError::new(any_integer_code(), dummy_tag());
        let cloned_error = original_error.clone();
        expect!(cloned_error.code).to_equal(original_error.code);
    }

    #[test]
    fn code__get_after_copied_when_enum__expect_same_value_as_original_error() {
        for code in each_enum_scenario() {
            let original_error = TaggedError::new(code, dummy_tag());
            let copied_error = original_error;
            expect!(copied_error.code).to_equal(original_error.code);
        }
    }

    #[test]
    fn code__get_after_cloned_when_enum__expect_same_value_as_original_error() {
        for code in each_enum_scenario() {
            let original_error = TaggedError::new(code, dummy_tag());
            let cloned_error = original_error.clone();
            expect!(cloned_error.code).to_equal(original_error.code);
        }
    }

    #[test]
    fn tag__get_after_new__expect_same_value_passed_to_constructor() {
        let tag = error_tag!("some error tag");
        let error = TaggedError::new(dummy_code(), tag);
        expect!(HalfUsize::from(error.tag)).to_equal(HalfUsize::from(tag));
    }

    fn dummy_code() -> NonZero<HalfUsize> {
        any_integer_code()
    }

    #[test]
    fn tag__get_after_copied__expect_same_value_as_original_error() {
        let original_error = TaggedError::new(any_integer_code(), error_tag!("oops"));
        let copied_error = original_error;
        expect!(HalfUsize::from(copied_error.tag)).to_equal(HalfUsize::from(original_error.tag));
    }

    #[test]
    fn tag__get_after_cloned__expect_same_value_as_original_error() {
        let original_error = TaggedError::new(any_integer_code(), error_tag!("another", "oops"));
        let cloned_error = original_error.clone();
        expect!(HalfUsize::from(cloned_error.tag)).to_equal(HalfUsize::from(original_error.tag));
    }

    #[test]
    fn from__called_for_instance_with_integer_code__expect_code_as_most_significant_half_of_usize() {
        let error = TaggedError::new(any_integer_code(), dummy_tag());
        expect!((NonZero::<usize>::from(error).get() >> HalfUsize::BITS) as HalfUsize).to_equal(error.code.get());
    }

    #[test]
    fn from__called_for_instance_with_enum_code__expect_code_as_most_significant_half_of_usize() {
        for code in each_enum_scenario() {
            let error = TaggedError::new(code, dummy_tag());
            expect!((NonZero::<usize>::from(error).get() >> HalfUsize::BITS) as HalfUsize).to_equal(NonZero::<HalfUsize>::from(error.code).get());
        }
    }

    #[test]
    fn from__called_for_instance__expect_tag_as_least_significant_half_of_usize() {
        let error = TaggedError::new(any_integer_code(), error_tag!("something something something error"));
        expect!(NonZero::<usize>::from(error).get() as HalfUsize).to_equal(HalfUsize::from(error.tag));
    }

    #[test]
    fn into__called_for_instance_with_integer_code__expect_code_as_most_significant_half_of_usize() {
        let error = TaggedError::new(any_integer_code(), dummy_tag());
        expect!((Into::<NonZero<usize>>::into(error).get() >> HalfUsize::BITS) as HalfUsize).to_equal(error.code.get());
    }

    #[test]
    fn into__called_for_instance_with_enum_code__expect_code_as_most_significant_half_of_usize() {
        for code in each_enum_scenario() {
            let error = TaggedError::new(code, dummy_tag());
            expect!((Into::<NonZero<usize>>::into(error).get() >> HalfUsize::BITS) as HalfUsize).to_equal(NonZero::<HalfUsize>::from(error.code).get());
        }
    }

    #[test]
    fn into__called_for_instance__expect_tag_as_least_significant_half_of_usize() {
        let error = TaggedError::new(any_integer_code(), error_tag!("Error McErrorFace"));
        expect!(Into::<NonZero<usize>>::into(error).get() as HalfUsize).to_equal(error.tag.into());
    }

    #[test]
    fn from__called_for_reference_with_integer_code__expect_code_as_most_significant_half_of_usize() {
        let error = TaggedError::new(any_integer_code(), dummy_tag());
        expect!((NonZero::<usize>::from(&error).get() >> HalfUsize::BITS) as HalfUsize).to_equal(error.code.get());
    }

    #[test]
    fn from__called_for_reference_with_enum_code__expect_code_as_most_significant_half_of_usize() {
        for code in each_enum_scenario() {
            let error = TaggedError::new(code, dummy_tag());
            expect!((NonZero::<usize>::from(&error).get() >> HalfUsize::BITS) as HalfUsize).to_equal(NonZero::<HalfUsize>::from(error.code).get());
        }
    }

    #[test]
    fn from__called_for_reference__expect_tag_as_least_significant_half_of_usize() {
        let error = TaggedError::new(any_integer_code(), error_tag!("something something something error"));
        expect!(NonZero::<usize>::from(&error).get() as HalfUsize).to_equal(HalfUsize::from(error.tag));
    }

    #[test]
    fn into__called_for_reference_with_integer_code__expect_code_as_most_significant_half_of_usize() {
        let error = TaggedError::new(any_integer_code(), dummy_tag());
        expect!((Into::<NonZero<usize>>::into(&error).get() >> HalfUsize::BITS) as HalfUsize).to_equal(error.code.get());
    }

    #[test]
    fn into__called_for_reference_with_enum_code__expect_code_as_most_significant_half_of_usize() {
        for code in each_enum_scenario() {
            let error = TaggedError::new(code, dummy_tag());
            expect!((Into::<NonZero<usize>>::into(&error).get() >> HalfUsize::BITS) as HalfUsize).to_equal(NonZero::<HalfUsize>::from(error.code).get());
        }
    }

    #[test]
    fn into__called_for_reference__expect_tag_as_least_significant_half_of_usize() {
        let error = TaggedError::new(any_integer_code(), error_tag!("Error McErrorFace"));
        expect!(Into::<NonZero<usize>>::into(&error).get() as HalfUsize).to_equal(error.tag.into());
    }
}

pub mod prelude {
    pub use super::TaggedError;
}
