#![allow(non_snake_case)]

use smeg_kernel::mem::mmio_register;

use fluent_test::prelude::*;

#[mmio_register]
#[datasheet("Document ID", "Section", 123)]
#[ro(rO_fIeLd, 0b_1_0000000000000000000000000000000)]
#[wo(wo_FiELD, 0b0_1_000000000000000000000000000000)]
#[rw(Rw_fieLD, 0b00_1_00000000000000000000000000000)]
#[xx(UNk_sBZ , 0b000_1_0000000000000000000000000000)]
#[xx(UNK_SBzP, 0b0000_1_000000000000000000000000000)]
#[xx(unK_sBO,  0b00000_1_00000000000000000000000000)]
#[xx(Unk_SBoP, 0b000000_1_0000000000000000000000000)]
#[xx(UnK_SBp,  0b0000000_1_000000000000000000000000)]
#[xx(SBz,      0b00000000_1_00000000000000000000000)]
#[xx(SbZp,     0b000000000_1_0000000000000000000000)]
#[xx(sBO,      0b0000000000_1_000000000000000000000)]
#[xx(SBop,     0b00000000000_1_00000000000000000000)]
#[xx(SbP,      0b000000000000_1_0000000000000000000)]
#[xx(Wi,       0b0000000000000_1_111111111111111111)]
struct RegisterFieldsWithNonRustCasing(u32);

#[test]
fn RO_FIELD_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RO_FIELD_MASK).to_equal(1 << 31);
}

#[test]
fn RO_FIELD_MSB__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RO_FIELD_MSB).to_equal(Some(31));
}

#[test]
fn RO_FIELD_LSB__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RO_FIELD_LSB).to_equal(Some(31));
}

#[test]
fn RO_FIELD_WIDTH__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RO_FIELD_WIDTH).to_equal(1);
}

#[test]
fn WO_FIELD_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::WO_FIELD_MASK).to_equal(1 << 30);
}

#[test]
fn WO_FIELD_MSB__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::WO_FIELD_MSB).to_equal(Some(30));
}

#[test]
fn WO_FIELD_LSB__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::WO_FIELD_LSB).to_equal(Some(30));
}

#[test]
fn WO_FIELD_WIDTH__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::WO_FIELD_WIDTH).to_equal(1);
}

#[test]
fn RW_FIELD_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RW_FIELD_MASK).to_equal(1 << 29);
}

#[test]
fn RW_FIELD_MSB__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RW_FIELD_MSB).to_equal(Some(29));
}

#[test]
fn RW_FIELD_LSB__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RW_FIELD_LSB).to_equal(Some(29));
}

#[test]
fn RW_FIELD_WIDTH__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RW_FIELD_WIDTH).to_equal(1);
}

#[test]
fn RESERVED_UNK_SBZ_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RESERVED_UNK_SBZ_MASK).to_equal(1 << 28);
}

#[test]
fn RESERVED_UNK_SBZP_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RESERVED_UNK_SBZP_MASK).to_equal(1 << 27);
}

#[test]
fn RESERVED_UNK_SBO_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RESERVED_UNK_SBO_MASK).to_equal(1 << 26);
}

#[test]
fn RESERVED_UNK_SBOP_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RESERVED_UNK_SBOP_MASK).to_equal(1 << 25);
}

#[test]
fn RESERVED_UNK_SBP_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RESERVED_UNK_SBP_MASK).to_equal(1 << 24);
}

#[test]
fn RESERVED_SBZ_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RESERVED_SBZ_MASK).to_equal(1 << 23);
}

#[test]
fn RESERVED_SBZP_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RESERVED_SBZP_MASK).to_equal(1 << 22);
}

#[test]
fn RESERVED_SBO_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RESERVED_SBO_MASK).to_equal(1 << 21);
}

#[test]
fn RESERVED_SBOP_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RESERVED_SBOP_MASK).to_equal(1 << 20);
}

#[test]
fn RESERVED_SBP_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RESERVED_SBP_MASK).to_equal(1 << 19);
}

#[test]
fn RESERVED_WI_MASK__get__expect_uppercase_rust_const_name() {
	expect!(RegisterFieldsWithNonRustCasing::RESERVED_WI_MASK).to_equal(0b1111111111111111111);
}
