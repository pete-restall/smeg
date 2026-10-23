#[doc = crate::docs::side_by_side_md!()]
use crate::docs;

#[doc = docs::side_by_side_md!("MemoryAttributes")]
pub unsafe trait MemoryAttributes { }

#[doc = docs::side_by_side_md!("Addressable")]
pub unsafe trait Addressable { type MemoryAttributes: MemoryAttributes; }

#[doc = docs::side_by_side_md!("Bank")]
pub unsafe trait Bank: Addressable { }

#[doc = docs::side_by_side_md!("CellPrimitive")]
pub unsafe trait CellPrimitive {
    type Type: Copy;

    const IS_READABLE: bool;
    const IS_WRITABLE: bool;

    const IS_READONLY: bool = Self::IS_READABLE && !Self::IS_WRITABLE;
    const IS_WRITEONLY: bool = !Self::IS_READABLE && Self::IS_WRITABLE;
}

#[doc = docs::side_by_side_md!("Cell")]
pub unsafe trait Cell: Addressable { type Primitive: CellPrimitive; }

#[doc = docs::side_by_side_md!("Readable")]
pub unsafe trait Readable { }

#[doc = docs::side_by_side_md!("Writable")]
pub unsafe trait Writable { }

mod banks;
pub use banks::prelude::*;

mod cells;
pub use cells::prelude::*;

pub use smeg_kernel_procmacro::mmio_register;

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use super::*;

    struct StubCellPrimitive<const R: bool, const W: bool>;
    unsafe impl<const R: bool, const W: bool> CellPrimitive for StubCellPrimitive<R, W> {
        type Type = u32;

        const IS_READABLE: bool = R;
        const IS_WRITABLE: bool = W;
    }

    type InaccessibleCell = StubCellPrimitive<false, false>;
    type ReadonlyCell = StubCellPrimitive<true, false>;
    type WriteonlyCell = StubCellPrimitive<false, true>;
    type ReadWriteCell = StubCellPrimitive<true, true>;

    #[test]
    fn IS_READONLY__get_when_not_readable_and_not_writable__expect_false() {
        expect!(InaccessibleCell::IS_READONLY).to_be_false();
    }

    #[test]
    fn IS_READONLY__get_when_readable_but_not_writable__expect_true() {
        expect!(ReadonlyCell::IS_READONLY).to_be_true();
    }

    #[test]
    fn IS_READONLY__get_when_not_readable_but_writable__expect_false() {
        expect!(WriteonlyCell::IS_READONLY).to_be_false();
    }

    #[test]
    fn IS_READONLY__get_when_readable_and_writable__expect_false() {
        expect!(ReadWriteCell::IS_READONLY).to_be_false();
    }

    #[test]
    fn IS_WRITEONLY__get_when_not_readable_and_not_writable__expect_false() {
        expect!(InaccessibleCell::IS_WRITEONLY).to_be_false();
    }

    #[test]
    fn IS_WRITEONLY__get_when_readable_but_not_writable__expect_false() {
        expect!(ReadonlyCell::IS_WRITEONLY).to_be_false();
    }

    #[test]
    fn IS_WRITEONLY__get_when_not_readable_but_writable__expect_true() {
        expect!(WriteonlyCell::IS_WRITEONLY).to_be_true();
    }

    #[test]
    fn IS_WRITEONLY__get_when_readable_and_writable__expect_false() {
        expect!(ReadWriteCell::IS_WRITEONLY).to_be_false();
    }
}

#[cfg(feature = "test_doubles")]
pub mod test_doubles {
    use crate::test_doubles::Dummy;

    use super::*;

    unsafe impl MemoryAttributes for Dummy { }

    unsafe impl Addressable for Dummy { type MemoryAttributes = Dummy; }

    unsafe impl Bank for Dummy { }

    unsafe impl Cell for Dummy { type Primitive = Dummy; }

    unsafe impl CellPrimitive for Dummy {
        type Type = usize;

        const IS_READABLE: bool = false;
        const IS_WRITABLE: bool = false;
    }
}
