#![allow(non_snake_case)]

use smeg_kernel::mem::mmio_register;

use fluent_test::prelude::*;

#[mmio_register]
#[datasheet("Document ID", "Section", 123)]
#[xx(UNK_SBZ , 0b1_000000_000_00_0000_000000_0000_00_0_0_0_0)]
#[xx(UNK_SBZP, 0b0_100110_000_00_0000_000000_0000_00_0_0_0_0)]
#[xx(UNK_SBO,  0b0_010001_000_00_0000_000000_0000_00_0_0_0_0)]
#[xx(UNK_SBOP, 0b0_001000_000_00_0000_000000_0000_00_0_0_0_0)]
#[xx(UNK_SBP,  0b0_000000_111_00_0000_000000_0000_00_0_0_0_0)]
#[xx(SBZ,      0b0_000000_000_11_0000_000000_0000_00_0_0_0_0)]
#[xx(SBZP,     0b0_000000_000_00_1111_000000_0000_00_0_0_0_0)]
#[xx(SBO,      0b0_000000_000_00_0000_111111_0000_00_0_0_0_0)]
#[xx(SBOP,     0b0_000000_000_00_0000_000000_1111_00_0_0_0_0)]
#[xx(SBP,      0b0_000000_000_00_0000_000000_0000_11_0_0_0_0)]
#[rw(FIELD_1,  0b0_000000_000_00_0000_000000_0000_00_1_0_0_0)]
#[ro(FIELD_2,  0b0_000000_000_00_0000_000000_0000_00_0_1_0_0)]
#[wo(FIELD_3,  0b0_000000_000_00_0000_000000_0000_00_0_0_1_0)]
#[xx(WI,       0b0_000000_000_00_0000_000000_0000_00_0_0_0_1)]
struct U32WithReservedBits(u32);

#[test]
fn repr__of_struct__expect_transparent_with_same_size_and_alignment_as_u32() {
    expect!(size_of::<U32WithReservedBits>()).to_equal(size_of::<u32>());
    expect!(align_of::<U32WithReservedBits>()).to_equal(align_of::<u32>());
}

#[test]
fn IS_READONLY__get__expect_false() {
    expect!(U32WithReservedBits::IS_READONLY).to_be_false();
}

#[test]
fn IS_READABLE__get__expect_true() {
    expect!(U32WithReservedBits::IS_READABLE).to_be_true();
}

#[test]
fn IS_WRITEONLY__get__expect_false() {
    expect!(U32WithReservedBits::IS_WRITEONLY).to_be_false();
}

#[test]
fn IS_WRITABLE__get__expect_true() {
    expect!(U32WithReservedBits::IS_WRITABLE).to_be_true();
}

#[test]
fn HAS_RESERVED_BITS__get__expect_true() {
    expect!(U32WithReservedBits::HAS_RESERVED_BITS).to_be_true();
}

#[test]
fn RESERVED_MASK__get__expect_or_of_individual_reserved_masks() {
    expect!(U32WithReservedBits::RESERVED_MASK).to_equal(
        U32WithReservedBits::RESERVED_UNK_SBZ_MASK |
        U32WithReservedBits::RESERVED_UNK_SBZP_MASK |
        U32WithReservedBits::RESERVED_UNK_SBO_MASK |
        U32WithReservedBits::RESERVED_UNK_SBOP_MASK |
        U32WithReservedBits::RESERVED_UNK_SBP_MASK |
        U32WithReservedBits::RESERVED_SBZ_MASK |
        U32WithReservedBits::RESERVED_SBZP_MASK |
        U32WithReservedBits::RESERVED_SBO_MASK |
        U32WithReservedBits::RESERVED_SBOP_MASK |
        U32WithReservedBits::RESERVED_SBP_MASK |
        U32WithReservedBits::RESERVED_WI_MASK);
}

#[test]
fn RESERVED_UNK_SBZ_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::RESERVED_UNK_SBZ_MASK).to_equal(1 << 31);
}

#[test]
fn RESERVED_UNK_SBZP_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::RESERVED_UNK_SBZP_MASK).to_equal(0b100110 << 25);
}

#[test]
fn RESERVED_UNK_SBO_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::RESERVED_UNK_SBO_MASK).to_equal(0b010001 << 25);
}

#[test]
fn RESERVED_UNK_SBOP_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::RESERVED_UNK_SBOP_MASK).to_equal(0b001000 << 25);
}

#[test]
fn RESERVED_UNK_SBP_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::RESERVED_UNK_SBP_MASK).to_equal(7 << 22);
}

#[test]
fn RESERVED_SBZ_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::RESERVED_SBZ_MASK).to_equal(3 << 20);
}

#[test]
fn RESERVED_SBZP_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::RESERVED_SBZP_MASK).to_equal(15 << 16);
}

#[test]
fn RESERVED_SBO_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::RESERVED_SBO_MASK).to_equal(0b111111 << 10);
}

#[test]
fn RESERVED_SBOP_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::RESERVED_SBOP_MASK).to_equal(15 << 6);
}

#[test]
fn RESERVED_SBP_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::RESERVED_SBP_MASK).to_equal(3 << 4);
}

#[test]
fn RESERVED_WI_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::RESERVED_WI_MASK).to_equal(1);
}

#[test]
fn FIELD_1_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::FIELD_1_MASK).to_equal(1 << 3);
}

#[test]
fn FIELD_1_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(U32WithReservedBits::FIELD_1_MSB).to_equal(Some(3));
}

#[test]
fn FIELD_1_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(U32WithReservedBits::FIELD_1_LSB).to_equal(Some(3));
}

#[test]
fn FIELD_1_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(U32WithReservedBits::FIELD_1_WIDTH).to_equal(1);
}

#[test]
fn FIELD_2_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::FIELD_2_MASK).to_equal(1 << 2);
}

#[test]
fn FIELD_2_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(U32WithReservedBits::FIELD_2_MSB).to_equal(Some(2));
}

#[test]
fn FIELD_2_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(U32WithReservedBits::FIELD_2_LSB).to_equal(Some(2));
}

#[test]
fn FIELD_2_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(U32WithReservedBits::FIELD_2_WIDTH).to_equal(1);
}

#[test]
fn FIELD_3_MASK__get__expect_same_as_defined_mask() {
    expect!(U32WithReservedBits::FIELD_3_MASK).to_equal(1 << 1);
}

#[test]
fn FIELD_3_MSB__get__expect_most_significant_bit_number_of_mask() {
    expect!(U32WithReservedBits::FIELD_3_MSB).to_equal(Some(1));
}

#[test]
fn FIELD_3_LSB__get__expect_least_significant_bit_number_of_mask() {
    expect!(U32WithReservedBits::FIELD_3_LSB).to_equal(Some(1));
}

#[test]
fn FIELD_3_WIDTH__get__expect_number_of_bits_between_msb_and_lsb() {
    expect!(U32WithReservedBits::FIELD_3_WIDTH).to_equal(1);
}
