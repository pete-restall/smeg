use core::mem::{align_of, size_of, MaybeUninit};

use crate::HalfUsize;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ErrorTag {
    id: HalfUsize
}

impl ErrorTag {
    pub fn new(elf_symbol: &'static MaybeUninit<u8>) -> ErrorTag {
        const {
            assert!(size_of::<ErrorTag>() == size_of::<HalfUsize>(), "Size of ErrorTag must be exactly half a machine word");
            assert!(align_of::<ErrorTag>() == align_of::<HalfUsize>(), "Alignment of ErrorTag must be the same as half a machine word");
        }

        ErrorTag { id: elf_symbol.as_ptr() as HalfUsize }
    }
}

impl From<ErrorTag> for HalfUsize {
    fn from(value: ErrorTag) -> Self {
        value.id
    }
}

impl From<&ErrorTag> for HalfUsize {
    fn from(value: &ErrorTag) -> Self {
        value.id
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use super::*;

    #[test]
    fn id__get_after_new__expect_least_significant_half_word_of_elf_symbol_pointer() {
        let elf_symbol = stub_elf_symbol();
        let tag = ErrorTag::new(elf_symbol);
        expect!(tag.id).to_equal(elf_symbol.as_ptr() as HalfUsize);
    }

    fn stub_elf_symbol() -> &'static MaybeUninit<u8> {
        static STUB: MaybeUninit<u8> = MaybeUninit::<u8>::new(0);
        &STUB
    }

    #[test]
    fn id__get_after_copied__expect_same_value_as_original_tag() {
        let original_tag = ErrorTag::new(stub_elf_symbol());
        let copied_tag = original_tag;
        expect!(copied_tag.id).to_equal(original_tag.id);
    }

    #[test]
    fn id__get_after_cloned__expect_same_value_as_original_tag() {
        let original_tag = ErrorTag::new(stub_elf_symbol());
        let cloned_tag = original_tag.clone();
        expect!(cloned_tag.id).to_equal(original_tag.id);
    }

    #[test]
    fn from__called_for_instance__expect_id() {
        let tag = ErrorTag::new(stub_elf_symbol());
        expect!(HalfUsize::from(tag)).to_equal(tag.id);
    }

    #[test]
    fn into__called_for_half_usize_with_instance__expect_id() {
        let tag = ErrorTag::new(stub_elf_symbol());
        expect!(Into::<HalfUsize>::into(tag)).to_equal(tag.id);
    }

    #[test]
    fn from__called_for_reference__expect_id() {
        let tag = ErrorTag::new(stub_elf_symbol());
        expect!(HalfUsize::from(&tag)).to_equal(tag.id);
    }

    #[test]
    fn into__called_for_half_usize_with_reference__expect_id() {
        let tag = &ErrorTag::new(stub_elf_symbol());
        expect!(Into::<HalfUsize>::into(tag)).to_equal(tag.id);
    }
}
