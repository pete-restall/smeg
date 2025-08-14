use crate::docs;

#[doc = docs::side_by_side_md!("try_slice_from")]
pub unsafe fn try_slice_from<'a, T: Sized>(start: *const T, past_end: *const T) -> Option<&'a [T]> {
    if size_of::<T>() == 0 {
        return None;
    }

    let start_addr = start as usize;
    let past_end_addr = past_end as usize;
    if start_addr == past_end_addr {
        return Some(&[]);
    }

    let alignment_mask = const { if align_of::<T>() == 0 { 0 } else { align_of::<T>() - 1 } };
    if start_addr > past_end_addr || start_addr == 0 || (start_addr & alignment_mask) != 0 || (past_end_addr & alignment_mask) != 0 {
        return None;
    }

    let addr_diff = past_end_addr - start_addr;
    let (number_of_elements, remainder) = (addr_diff / size_of::<T>(), addr_diff % size_of::<T>());
    if remainder == 0 {
        Some(unsafe { core::slice::from_raw_parts(start, number_of_elements) })
    } else {
        None
    }
}

#[doc = docs::side_by_side_md!("slice_from_unchecked")]
pub unsafe fn slice_from_unchecked<'a, T: Sized>(start: *const T, past_end: *const T) -> &'a [T] {
    const { assert!(size_of::<T>() > 0, "Cannot create a slice from a Zero-Sized Type (ZST)"); }

    let start_addr = start as usize;
    let past_end_addr = past_end as usize;
    unsafe {
        let number_of_elements = (past_end_addr - start_addr) / size_of::<T>();
        core::slice::from_raw_parts(start, number_of_elements)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use super::*;

    #[test]
    fn slice_from_unchecked__called_with_past_end_same_as_start__expect_empty_slice_is_returned() {
        let dummy = 0;
        let ptr = &raw const dummy;
        expect!(unsafe { slice_from_unchecked(ptr, ptr) }).to_be_empty();
    }

    #[test]
    fn slice_from_unchecked__called__expect_slice_is_returned() {
        let dummy = [1, 2, 3, 4, 5];
        let start = &raw const dummy[0];
        let past_end = &raw const dummy[4];
        expect!(unsafe { slice_from_unchecked(start, past_end) }).to_equal(&dummy[0..4]);
    }

    #[test]
    fn try_slice_from__called_with_past_end_same_as_start__expect_some_empty_slice_is_returned() {
        let dummy = 0;
        let ptr = &raw const dummy;
        expect!(unsafe { try_slice_from(ptr, ptr) }).to_equal(Some(&[]));
    }

    #[test]
    fn try_slice_from__called_with_two_nulls__expect_some_empty_slice_is_returned() {
        let null_ptr: *const u8 = core::ptr::null();
        expect!(unsafe { try_slice_from(null_ptr, null_ptr) }).to_equal(Some(&[]));
    }

    #[test]
    fn try_slice_from__called__expect_some_slice_is_returned() {
        let dummy = ['a', 'b', 'c', 'd', 'e'];
        let start = &raw const dummy[0];
        let past_end = &raw const dummy[4];
        expect!(unsafe { try_slice_from(start, past_end) }).to_equal(Some(&dummy[0..4]));
    }

    #[test]
    fn try_slice_from__called_with_byte_pointers__expect_some_slice_is_returned() {
        let dummy = [1_u8, 2_u8, 3_u8, 4_u8];
        let start = &raw const dummy[0];
        let past_end = &raw const dummy[3];
        expect!(unsafe { try_slice_from(start, past_end) }).to_equal(Some(&dummy[0..3]));
    }

    #[test]
    fn try_slice_from__called_with_past_end_before_start__expect_none_is_returned() {
        let dummy = [4, 2];
        let start = &raw const dummy[0];
        let past_end = &raw const dummy[1];
        expect!(unsafe { try_slice_from(past_end, start) }).to_equal(None);
    }

    #[test]
    fn try_slice_from__called_with_null_start__expect_none_is_returned() {
        let dummy = [2, 4, 6, 8];
        let past_end = &raw const dummy[3];
        expect!(unsafe { try_slice_from(core::ptr::null(), past_end) }).to_equal(None);
    }

    #[test]
    fn try_slice_from__called_with_null_past_end__expect_none_is_returned() {
        let dummy = [2, 4, 6, 8];
        let start = &raw const dummy[2];
        expect!(unsafe { try_slice_from(start, core::ptr::null()) }).to_equal(None);
    }

    #[test]
    fn try_slice_from__called_with_zero_sized_type__expect_none_is_returned() {
        let dummy = [0, 0];
        let ptr = unsafe { (&raw const dummy[0]).offset(1) };
        expect!(unsafe { try_slice_from(ptr, ptr) }).to_equal(Some(&[]));
    }

    #[test]
    fn try_slice_from__called_with_unaligned_but_equal_start_and_past_end__expect_some_empty_slice_is_returned() {
        let null_ptr: *const u8 = core::ptr::null();
        expect!(unsafe { try_slice_from(null_ptr, null_ptr) }).to_equal(Some(&[]));
    }

    #[test]
    fn try_slice_from__called_with_unaligned_start__expect_none_is_returned() {
        #[repr(C, align(32))]
        struct Big([u8; 32]);

        _try_slice_from__called_with_unaligned_start__expect_none_is_returned(&[1_u16, 2_u16]);
        _try_slice_from__called_with_unaligned_start__expect_none_is_returned(&[1_usize, 2_usize]);
        _try_slice_from__called_with_unaligned_start__expect_none_is_returned(&[Big([1; 32]), Big([2; 32])]);
    }

    fn _try_slice_from__called_with_unaligned_start__expect_none_is_returned<T: Sized>(dummy: &[T; 2]) {
        let start = &raw const dummy[0];
        let past_end = &raw const dummy[1];
        try_slice_from__called_with_unaligned_pointer__expect::<T, _>(|offset| {
            expect!(unsafe { try_slice_from(start.byte_offset(offset), past_end) }.is_none()).to_be_true();
        });
    }

    fn try_slice_from__called_with_unaligned_pointer__expect<T: Sized, F: Fn(isize)>(assertion: F) {
        let bad_alignment = align_of::<T>() as isize - 1;
        for offset in -bad_alignment..=bad_alignment {
            if offset != 0 {
                assertion(offset);
            }
        }
    }

    #[test]
    fn try_slice_from__called_with_unaligned_past_end__expect_none_is_returned() {
        #[repr(C, align(64))]
        struct Big([u8; 64]);

        _try_slice_from__called_with_unaligned_past_end__expect_none_is_returned(&[2_u16, 1_u16]);
        _try_slice_from__called_with_unaligned_past_end__expect_none_is_returned(&[100_isize, -200_isize]);
        _try_slice_from__called_with_unaligned_past_end__expect_none_is_returned(&[Big([1; 64]), Big([2; 64])]);
    }

    fn _try_slice_from__called_with_unaligned_past_end__expect_none_is_returned<T: Sized>(dummy: &[T; 2]) {
        let start = &raw const dummy[0];
        let past_end = &raw const dummy[1];
        try_slice_from__called_with_unaligned_pointer__expect::<T, _>(|offset| {
            expect!(unsafe { try_slice_from(start, past_end.byte_offset(offset)) }.is_none()).to_be_true();
        });
    }

    #[test]
    fn try_slice_from__called_with_aligned_but_fractional_number_of_elements_for_start__expect_none_is_returned() {
        #[repr(C, align(4))]
        struct Dummy([u8; 8]);
        expect!(align_of::<Dummy>()).to_equal(4);

        let dummy = [Dummy([0; 8]), Dummy([0; 8]), Dummy([0; 8])];
        let start = &raw const dummy[0];
        let past_end = &raw const dummy[2];
        expect!(unsafe { try_slice_from(start.byte_offset(4), past_end) }.is_none()).to_be_true();
    }

    #[test]
    fn try_slice_from__called_with_aligned_but_fractional_number_of_elements_for_past_end__expect_none_is_returned() {
        #[repr(C, align(4))]
        struct Dummy([u8; 8]);
        expect!(align_of::<Dummy>()).to_equal(4);

        let dummy = [Dummy([0; 8]), Dummy([0; 8]), Dummy([0; 8])];
        let start = &raw const dummy[0];
        let past_end = &raw const dummy[2];
        expect!(unsafe { try_slice_from(start, past_end.byte_offset(-4)) }.is_none()).to_be_true();
    }
}
