#![allow(non_snake_case)]

use smeg_kernel::mem::mmio_register;

use fluent_test::prelude::*;

#[mmio_register]
#[datasheet("Document ID", "Section", 123)]
#[rw(FIELD_1,   0b1_00_000000000_00000000000000000000)]
#[rw(FIELD_2,   0b0_11_000000000_00000000000000000000)]
#[rw(OVERLAP_1, 0b0_00_111011010_00000000000000000000)]
#[rw(OVERLAP_2, 0b0_00_000100101_00000000000000000000)]
#[rw(FIELD_3,   0b0_00_000000000_11111111111111111111)]
struct ReadWriteU32WithoutReservedBits(u32);

#[test]
fn repr__of_struct__expect_transparent_with_same_size_and_alignment_as_u32() {
    expect!(size_of::<ReadWriteU32WithoutReservedBits>()).to_equal(size_of::<u32>());
    expect!(align_of::<ReadWriteU32WithoutReservedBits>()).to_equal(align_of::<u32>());
}

#[test]
fn IS_READONLY__get__expect_false() {
    expect!(ReadWriteU32WithoutReservedBits::IS_READONLY).to_be_false();
}

#[test]
fn IS_READABLE__get__expect_true() {
    expect!(ReadWriteU32WithoutReservedBits::IS_READABLE).to_be_true();
}

#[test]
fn IS_WRITEONLY__get__expect_false() {
    expect!(ReadWriteU32WithoutReservedBits::IS_WRITEONLY).to_be_false();
}

#[test]
fn IS_WRITABLE__get__expect_true() {
    expect!(ReadWriteU32WithoutReservedBits::IS_WRITABLE).to_be_true();
}

#[test]
fn FIELD_1_MASK__get__expect_same_as_defined_mask() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_1_MASK).to_equal(1 << 31);
}

#[test]
fn FIELD_1_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_1_MSB).to_equal(Some(31));
}

#[test]
fn FIELD_1_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_1_LSB).to_equal(Some(31));
}

#[test]
fn FIELD_1_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_1_WIDTH).to_equal(1);
}

#[test]
fn FIELD_2_MASK__get__expect_same_as_defined_mask() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_2_MASK).to_equal(3 << 29);
}

#[test]
fn FIELD_2_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_2_MSB).to_equal(Some(30));
}

#[test]
fn FIELD_2_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_2_LSB).to_equal(Some(29));
}

#[test]
fn FIELD_2_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_2_WIDTH).to_equal(2);
}

#[test]
fn OVERLAP_1_MASK__get__expect_same_as_defined_mask() {
    expect!(ReadWriteU32WithoutReservedBits::OVERLAP_1_MASK).to_equal(0b111011010 << 20);
}

#[test]
fn OVERLAP_1_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(ReadWriteU32WithoutReservedBits::OVERLAP_1_MSB).to_equal(Some(28));
}

#[test]
fn OVERLAP_1_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(ReadWriteU32WithoutReservedBits::OVERLAP_1_LSB).to_equal(Some(21));
}

#[test]
fn OVERLAP_1_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(ReadWriteU32WithoutReservedBits::OVERLAP_1_WIDTH).to_equal(8);
}

#[test]
fn OVERLAP_2_MASK__get__expect_same_as_defined_mask() {
    expect!(ReadWriteU32WithoutReservedBits::OVERLAP_2_MASK).to_equal(0b000100101 << 20);
}

#[test]
fn OVERLAP_2_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(ReadWriteU32WithoutReservedBits::OVERLAP_2_MSB).to_equal(Some(25));
}

#[test]
fn OVERLAP_2_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(ReadWriteU32WithoutReservedBits::OVERLAP_2_LSB).to_equal(Some(20));
}

#[test]
fn OVERLAP_2_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(ReadWriteU32WithoutReservedBits::OVERLAP_2_WIDTH).to_equal(6);
}

#[test]
fn FIELD_3_MASK__get__expect_same_as_defined_mask() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_3_MASK).to_equal((1 << 20) - 1);
}

#[test]
fn FIELD_3_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_3_MSB).to_equal(Some(19));
}

#[test]
fn FIELD_3_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_3_LSB).to_equal(Some(0));
}

#[test]
fn FIELD_3_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(ReadWriteU32WithoutReservedBits::FIELD_3_WIDTH).to_equal(20);
}

#[test]
fn HAS_RESERVED_BITS__get__expect_false() {
    expect!(ReadWriteU32WithoutReservedBits::HAS_RESERVED_BITS).to_be_false();
}

#[test]
fn RESERVED_MASK__get__expect_or_of_individual_reserved_masks() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_MASK).to_equal(
        ReadWriteU32WithoutReservedBits::RESERVED_UNK_SBZ_MASK |
        ReadWriteU32WithoutReservedBits::RESERVED_UNK_SBZP_MASK |
        ReadWriteU32WithoutReservedBits::RESERVED_UNK_SBO_MASK |
        ReadWriteU32WithoutReservedBits::RESERVED_UNK_SBOP_MASK |
        ReadWriteU32WithoutReservedBits::RESERVED_UNK_SBP_MASK |
        ReadWriteU32WithoutReservedBits::RESERVED_SBZ_MASK |
        ReadWriteU32WithoutReservedBits::RESERVED_SBZP_MASK |
        ReadWriteU32WithoutReservedBits::RESERVED_SBO_MASK |
        ReadWriteU32WithoutReservedBits::RESERVED_SBOP_MASK |
        ReadWriteU32WithoutReservedBits::RESERVED_SBP_MASK |
        ReadWriteU32WithoutReservedBits::RESERVED_WI_MASK);
}

#[test]
fn RESERVED_UNK_SBZ_MASK__get__expect_zero() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_UNK_SBZ_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBZP_MASK__get__expect_zero() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_UNK_SBZP_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBO_MASK__get__expect_zero() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_UNK_SBO_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBOP_MASK__get__expect_zero() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_UNK_SBOP_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBP_MASK__get__expect_zero() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_UNK_SBP_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBZ_MASK__get__expect_zero() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_SBZ_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBZP_MASK__get__expect_zero() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_SBZP_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBO_MASK__get__expect_zero() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_SBO_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBOP_MASK__get__expect_zero() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_SBOP_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBP_MASK__get__expect_zero() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_SBP_MASK).to_equal(0);
}

#[test]
fn RESERVED_WI_MASK__get__expect_zero() {
    expect!(ReadWriteU32WithoutReservedBits::RESERVED_WI_MASK).to_equal(0);
}
