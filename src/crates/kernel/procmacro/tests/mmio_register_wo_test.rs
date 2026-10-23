#![allow(non_snake_case)]

use smeg_kernel::mem::{CellPrimitive, mmio_register};

use fluent_test::prelude::*;

#[mmio_register]
#[datasheet("Document ID", "Section", 123)]
#[wo(FIELD_1,   0b11_0_0000_000000000)]
#[wo(FIELD_2,   0b00_1_0000_000000000)]
#[wo(OVERLAP_1, 0b00_0_1010_000000000)]
#[wo(OVERLAP_2, 0b00_0_0101_000000000)]
#[wo(FIELD_3,   0b00_0_0000_111111111)]
struct WriteonlyU16WithoutReservedBits(u16);

#[test]
fn repr__of_struct__expect_transparent_with_same_size_and_alignment_as_u16() {
    expect!(size_of::<WriteonlyU16WithoutReservedBits>()).to_equal(size_of::<u16>());
    expect!(align_of::<WriteonlyU16WithoutReservedBits>()).to_equal(align_of::<u16>());
}

#[test]
fn IS_READONLY__get__expect_false() {
    expect!(WriteonlyU16WithoutReservedBits::IS_READONLY).to_be_false();
}

#[test]
fn IS_READABLE__get__expect_false() {
    expect!(WriteonlyU16WithoutReservedBits::IS_READABLE).to_be_false();
}

#[test]
fn IS_WRITEONLY__get__expect_true() {
    expect!(WriteonlyU16WithoutReservedBits::IS_WRITEONLY).to_be_true();
}

#[test]
fn IS_WRITABLE__get__expect_true() {
    expect!(WriteonlyU16WithoutReservedBits::IS_WRITABLE).to_be_true();
}

#[test]
fn FIELD_1_MASK__get__expect_same_as_defined_mask() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_1_MASK).to_equal(3 << 14);
}

#[test]
fn FIELD_1_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_1_MSB).to_equal(Some(15));
}

#[test]
fn FIELD_1_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_1_LSB).to_equal(Some(14));
}

#[test]
fn FIELD_1_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_1_WIDTH).to_equal(2);
}

#[test]
fn FIELD_2_MASK__get__expect_same_as_defined_mask() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_2_MASK).to_equal(1 << 13);
}

#[test]
fn FIELD_2_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_2_MSB).to_equal(Some(13));
}

#[test]
fn FIELD_2_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_2_LSB).to_equal(Some(13));
}

#[test]
fn FIELD_2_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_2_WIDTH).to_equal(1);
}

#[test]
fn OVERLAP_1_MASK__get__expect_same_as_defined_mask() {
    expect!(WriteonlyU16WithoutReservedBits::OVERLAP_1_MASK).to_equal(0b1010 << 9);
}

#[test]
fn OVERLAP_1_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(WriteonlyU16WithoutReservedBits::OVERLAP_1_MSB).to_equal(Some(12));
}

#[test]
fn OVERLAP_1_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(WriteonlyU16WithoutReservedBits::OVERLAP_1_LSB).to_equal(Some(10));
}

#[test]
fn OVERLAP_1_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(WriteonlyU16WithoutReservedBits::OVERLAP_1_WIDTH).to_equal(3);
}

#[test]
fn OVERLAP_2_MASK__get__expect_same_as_defined_mask() {
    expect!(WriteonlyU16WithoutReservedBits::OVERLAP_2_MASK).to_equal(0b0101 << 9);
}

#[test]
fn OVERLAP_2_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(WriteonlyU16WithoutReservedBits::OVERLAP_2_MSB).to_equal(Some(11));
}

#[test]
fn OVERLAP_2_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(WriteonlyU16WithoutReservedBits::OVERLAP_2_LSB).to_equal(Some(9));
}

#[test]
fn OVERLAP_2_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(WriteonlyU16WithoutReservedBits::OVERLAP_2_WIDTH).to_equal(3);
}

#[test]
fn FIELD_3_MASK__get__expect_same_as_defined_mask() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_3_MASK).to_equal(0x1ff);
}

#[test]
fn FIELD_3_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_3_MSB).to_equal(Some(8));
}

#[test]
fn FIELD_3_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_3_LSB).to_equal(Some(0));
}

#[test]
fn FIELD_3_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(WriteonlyU16WithoutReservedBits::FIELD_3_WIDTH).to_equal(9);
}

#[test]
fn HAS_RESERVED_BITS__get__expect_false() {
    expect!(WriteonlyU16WithoutReservedBits::HAS_RESERVED_BITS).to_be_false();
}

#[test]
fn RESERVED_MASK__get__expect_or_of_individual_reserved_masks() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_MASK).to_equal(
        WriteonlyU16WithoutReservedBits::RESERVED_UNK_SBZ_MASK |
        WriteonlyU16WithoutReservedBits::RESERVED_UNK_SBZP_MASK |
        WriteonlyU16WithoutReservedBits::RESERVED_UNK_SBO_MASK |
        WriteonlyU16WithoutReservedBits::RESERVED_UNK_SBOP_MASK |
        WriteonlyU16WithoutReservedBits::RESERVED_UNK_SBP_MASK |
        WriteonlyU16WithoutReservedBits::RESERVED_SBZ_MASK |
        WriteonlyU16WithoutReservedBits::RESERVED_SBZP_MASK |
        WriteonlyU16WithoutReservedBits::RESERVED_SBO_MASK |
        WriteonlyU16WithoutReservedBits::RESERVED_SBOP_MASK |
        WriteonlyU16WithoutReservedBits::RESERVED_SBP_MASK |
        WriteonlyU16WithoutReservedBits::RESERVED_WI_MASK);
}

#[test]
fn RESERVED_UNK_SBZ_MASK__get__expect_zero() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_UNK_SBZ_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBZP_MASK__get__expect_zero() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_UNK_SBZP_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBO_MASK__get__expect_zero() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_UNK_SBO_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBOP_MASK__get__expect_zero() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_UNK_SBOP_MASK).to_equal(0);
}

#[test]
fn RESERVED_UNK_SBP_MASK__get__expect_zero() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_UNK_SBP_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBZ_MASK__get__expect_zero() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_SBZ_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBZP_MASK__get__expect_zero() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_SBZP_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBO_MASK__get__expect_zero() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_SBO_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBOP_MASK__get__expect_zero() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_SBOP_MASK).to_equal(0);
}

#[test]
fn RESERVED_SBP_MASK__get__expect_zero() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_SBP_MASK).to_equal(0);
}

#[test]
fn RESERVED_WI_MASK__get__expect_zero() {
    expect!(WriteonlyU16WithoutReservedBits::RESERVED_WI_MASK).to_equal(0);
}
