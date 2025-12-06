#![allow(non_snake_case)]

use smeg_kernel::mem::mmio_register;

use fluent_test::prelude::*;

#[mmio_register]
#[datasheet("Document ID", "Section", 123)]
#[ro(FIELD_1,   0b1_00_000000000_00000000000000000000)]
#[ro(FIELD_2,   0b0_11_000000000_00000000000000000000)]
#[ro(OVERLAP_1, 0b0_00_111011010_00000000000000000000)]
#[ro(OVERLAP_2, 0b0_00_000100101_00000000000000000000)]
#[ro(FIELD_3,   0b0_00_000000000_11111111111111111111)]
struct ReadonlyU32WithoutReservedBits(u32);

#[test]
fn repr__of_struct__expect_transparent_with_same_size_and_alignment_as_u32() {
    expect!(size_of::<ReadonlyU32WithoutReservedBits>()).to_equal(size_of::<u32>());
    expect!(align_of::<ReadonlyU32WithoutReservedBits>()).to_equal(align_of::<u32>());
}

#[test]
fn IS_READONLY__get__expect_true() {
    expect!(ReadonlyU32WithoutReservedBits::IS_READONLY).to_be_true();
}

#[test]
fn IS_READABLE__get__expect_true() {
    expect!(ReadonlyU32WithoutReservedBits::IS_READABLE).to_be_true();
}

#[test]
fn IS_WRITEONLY__get__expect_false() {
    expect!(ReadonlyU32WithoutReservedBits::IS_WRITEONLY).to_be_false();
}

#[test]
fn IS_WRITABLE__get__expect_false() {
    expect!(ReadonlyU32WithoutReservedBits::IS_WRITABLE).to_be_false();
}

#[test]
fn FIELD_1_MASK__get__expect_same_as_defined_mask() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_1_MASK).to_equal(1 << 31);
}

#[test]
fn FIELD_1_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_1_MSB).to_equal(Some(31));
}

#[test]
fn FIELD_1_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_1_LSB).to_equal(Some(31));
}

#[test]
fn FIELD_1_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_1_WIDTH).to_equal(1);
}

#[test]
fn FIELD_2_MASK__get__expect_same_as_defined_mask() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_2_MASK).to_equal(3 << 29);
}

#[test]
fn FIELD_2_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_2_MSB).to_equal(Some(30));
}

#[test]
fn FIELD_2_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_2_LSB).to_equal(Some(29));
}

#[test]
fn FIELD_2_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_2_WIDTH).to_equal(2);
}

#[test]
fn OVERLAP_1_MASK__get__expect_same_as_defined_mask() {
    expect!(ReadonlyU32WithoutReservedBits::OVERLAP_1_MASK).to_equal(0b111011010 << 20);
}

#[test]
fn OVERLAP_1_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(ReadonlyU32WithoutReservedBits::OVERLAP_1_MSB).to_equal(Some(28));
}

#[test]
fn OVERLAP_1_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(ReadonlyU32WithoutReservedBits::OVERLAP_1_LSB).to_equal(Some(21));
}

#[test]
fn OVERLAP_1_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(ReadonlyU32WithoutReservedBits::OVERLAP_1_WIDTH).to_equal(8);
}

#[test]
fn OVERLAP_2_MASK__get__expect_same_as_defined_mask() {
    expect!(ReadonlyU32WithoutReservedBits::OVERLAP_2_MASK).to_equal(0b000100101 << 20);
}

#[test]
fn OVERLAP_2_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(ReadonlyU32WithoutReservedBits::OVERLAP_2_MSB).to_equal(Some(25));
}

#[test]
fn OVERLAP_2_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(ReadonlyU32WithoutReservedBits::OVERLAP_2_LSB).to_equal(Some(20));
}

#[test]
fn OVERLAP_2_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(ReadonlyU32WithoutReservedBits::OVERLAP_2_WIDTH).to_equal(6);
}

#[test]
fn FIELD_3_MASK__get__expect_same_as_defined_mask() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_3_MASK).to_equal((1 << 20) - 1);
}

#[test]
fn FIELD_3_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_3_MSB).to_equal(Some(19));
}

#[test]
fn FIELD_3_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_3_LSB).to_equal(Some(0));
}

#[test]
fn FIELD_3_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(ReadonlyU32WithoutReservedBits::FIELD_3_WIDTH).to_equal(20);
}

#[test]
fn HAS_RESERVED_BITS__get__expect_false() {
    expect!(ReadonlyU32WithoutReservedBits::HAS_RESERVED_BITS).to_be_false();
}

#[test]
fn RESERVED_MASK__get__expect_or_of_individual_reserved_masks() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_MASK).to_equal(
        ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBZ_MASK |
        ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBZP_MASK |
        ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBO_MASK |
        ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBOP_MASK |
        ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBP_MASK |
        ReadonlyU32WithoutReservedBits::RESERVED_SBZ_MASK |
        ReadonlyU32WithoutReservedBits::RESERVED_SBZP_MASK |
        ReadonlyU32WithoutReservedBits::RESERVED_SBO_MASK |
        ReadonlyU32WithoutReservedBits::RESERVED_SBOP_MASK |
        ReadonlyU32WithoutReservedBits::RESERVED_SBP_MASK |
        ReadonlyU32WithoutReservedBits::RESERVED_WI_MASK);
}

#[test]
fn RESERVED_UNK_SBZ_MASK__get__expect_zero() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBZ_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBZP_MASK__get__expect_zero() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBZP_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBO_MASK__get__expect_zero() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBO_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBOP_MASK__get__expect_zero() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBOP_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBP_MASK__get__expect_zero() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBP_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBZ_MASK__get__expect_zero() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_SBZ_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBZP_MASK__get__expect_zero() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_SBZP_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBO_MASK__get__expect_zero() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_SBO_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBOP_MASK__get__expect_zero() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_SBOP_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBP_MASK__get__expect_zero() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_SBP_MASK).to_equal(0);
}

#[test]
fn RESERVED_WI_MASK__get__expect_zero() {
    expect!(ReadonlyU32WithoutReservedBits::RESERVED_WI_MASK).to_equal(0);
}

// TODO: another suite of tests for write-only, read-write and testing fields defined 'sOmE_WEIrd_CasING' -> 'SOME_WEIRD_CASING'



// TODO: These are the tests we'll want...
//
// expect ReadonlyU32WithoutReservedBits::MSB_FIELD_MASK == 0x80000000
// expect ReadonlyU32WithoutReservedBits::MSB_FIELD_MSB == 31
// expect ReadonlyU32WithoutReservedBits::MSB_FIELD_LSB == 31
// expect ReadonlyU32WithoutReservedBits::MSB_FIELD_WIDTH == 1
// ... repeat for the other fields

// expect ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBZ == 0
// expect ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBZP == 0
// expect ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBO == 0
// expect ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBOP == 0
// expect ReadonlyU32WithoutReservedBits::RESERVED_UNK_SBP == 0

// expect ReadonlyU32WithoutReservedBits::RESERVED_SBZ == 0
// expect ReadonlyU32WithoutReservedBits::RESERVED_SBZP == 0
// expect ReadonlyU32WithoutReservedBits::RESERVED_SBO == 0
// expect ReadonlyU32WithoutReservedBits::RESERVED_SBOP == 0
// expect ReadonlyU32WithoutReservedBits::RESERVED_SBP == 0
// expect ReadonlyU32WithoutReservedBits::RESERVED_WI == 0

// expect method ReadonlyU32WithoutReservedBitsAccessor.get() to return the entire value

// expect method ReadonlyU32WithoutReservedBitsAccessor.msb_field() to return unshifted value
// expect method ReadonlyU32WithoutReservedBitsAccessor.msb_field_ra() to return right-aligned value
// expect method ReadonlyU32WithoutReservedBitsAccessor.msb_field_se() to return sign-extended right-aligned value
// expect method ReadonlyU32WithoutReservedBitsAccessor.msb_field_la() to return left-aligned value

// for registers that can be written to, things get more interesting...llsc
//     - we can only provide a 'set_unchecked(value)' to set the entire register, because 'SB?P' values need writing depending on _whether they've been read before or not_.
//       (ARM Glossary, 105565_0200_02_en - this version does not explicitly state it, but the ARMv6 glossary even stipulates the value written must correspond to the value
//       previously read by the _same core_).  The accessors have no way to know about this context, whether it's the first write or a subsequent write-preserved.
//     - setting all fields together (some sort of 'rmw(|value| ...)' method, which can mask and set reserved bits automatically) - HOWEVER; need to think about how to do
//       this atomically, such as 'try_rmw', LL/SC or CAS (_with_ configurable number of retries, maybe ?)
//     - setting any individual field (some sort of 'field_rmw(|value| ...)' method that does the appropriate masking, plus 'field_rmw_la'; 'field_rmw_ra' and
//       'field_rmw_signed' are un-necessary).  Same concerns as above - LL/SC, CAS, whatever.  In fact, the setters should be based off trait implementations of the
//       underlying CellAccessor that the RegAccessor will need to use (impl RegAccessor<'mem, C> { pub fn whatever_llsc_rmw(...) where C::Type: LlscReadModifyWrite { ... } } )
//     - Maybe we only provide 'set_unchecked' and let the higher-level code handle locking, or atomicity ?
//     - Do we want const versions of the setters, to avoid a lot of runtime masking when, for example, setting initial (non-reserved) values ?
