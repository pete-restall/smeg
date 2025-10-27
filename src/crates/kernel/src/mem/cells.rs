use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use crate::docs;

use super::{Addressable, Cell, CellPrimitive, MemoryAttributes, Readable, Writable};

macro_rules! impl_cell_traits_for {
    ($name:ident) => {
        unsafe impl<M: MemoryAttributes, P: CellPrimitive> Cell for $name<M, P> { type Primitive = P; }
        unsafe impl<M: MemoryAttributes, P: CellPrimitive> Addressable for $name<M, P> { type MemoryAttributes = M; }
        impl<M, P> !Send for $name<M, P> { }
        impl<M, P> !Sync for $name<M, P> { }
        impl<M, P> !Deref for $name<M, P> { }
        impl<M, P> !DerefMut for $name<M, P> { }
    };
}

#[repr(transparent)]
#[doc = docs::side_by_side_md!("ReadWriteCell")]
pub struct ReadWriteCell<M: MemoryAttributes, P: CellPrimitive> {
    value: P,
    _attributes: PhantomData<M>
}

impl<M: MemoryAttributes, P: CellPrimitive> ReadWriteCell<M, P> {
    const _ENSURE_LAYOUT_OF_UNDERLYING_TYPE: () = {
        assert!(size_of::<ReadWriteCell<M, P>>() == size_of::<P>(), "Size of ReadWriteCell<M, P> must be the same size as P");
        assert!(align_of::<ReadWriteCell<M, P>>() == align_of::<P>(), "Alignment of ReadWriteCell<M, P> must be the same as P");

        assert!(size_of::<P>() == size_of::<P::Type>(), "Size of P (a CellPrimitive) must be the same size as its encapsulated Type");
        assert!(align_of::<P>() == align_of::<P::Type>(), "Alignment of P (a CellPrimitive) must be the same as its encapsulated Type");
    };
}

unsafe impl<M: MemoryAttributes, P: CellPrimitive> Readable for ReadWriteCell<M, P> { }
unsafe impl<M: MemoryAttributes, P: CellPrimitive> Writable for ReadWriteCell<M, P> { }
impl_cell_traits_for!(ReadWriteCell);

#[repr(transparent)]
#[doc = docs::side_by_side_md!("ReadonlyCell")]
pub struct ReadonlyCell<M: MemoryAttributes, P: CellPrimitive> {
    value: P,
    _attributes: PhantomData<M>
}

impl<M: MemoryAttributes, P: CellPrimitive> ReadonlyCell<M, P> {
    const _ENSURE_LAYOUT_OF_UNDERLYING_TYPE: () = {
        assert!(size_of::<ReadonlyCell<M, P>>() == size_of::<P>(), "Size of ReadonlyCell<M, P> must be the same size as P");
        assert!(align_of::<ReadonlyCell<M, P>>() == align_of::<P>(), "Alignment of ReadonlyCell<M, P> must be the same as P");

        assert!(size_of::<P>() == size_of::<P::Type>(), "Size of P (a CellPrimitive) must be the same size as its encapsulated Type");
        assert!(align_of::<P>() == align_of::<P::Type>(), "Alignment of P (a CellPrimitive) must be the same as its encapsulated Type");
    };
}

unsafe impl<M: MemoryAttributes, P: CellPrimitive> Readable for ReadonlyCell<M, P> { }
impl_cell_traits_for!(ReadonlyCell);

#[repr(transparent)]
#[doc = docs::side_by_side_md!("WriteonlyCell")]
pub struct WriteonlyCell<M: MemoryAttributes, P: CellPrimitive> {
    value: P,
    _attributes: PhantomData<M>
}

impl<M: MemoryAttributes, P: CellPrimitive> WriteonlyCell<M, P> {
    const _ENSURE_LAYOUT_OF_UNDERLYING_TYPE: () = {
        assert!(size_of::<WriteonlyCell<M, P>>() == size_of::<P>(), "Size of WriteonlyCell<M, P> must be the same size as P");
        assert!(align_of::<WriteonlyCell<M, P>>() == align_of::<P>(), "Alignment of WriteonlyCell<M, P> must be the same as P");

        assert!(size_of::<P>() == size_of::<P::Type>(), "Size of P (a CellPrimitive) must be the same size as its encapsulated Type");
        assert!(align_of::<P>() == align_of::<P::Type>(), "Alignment of P (a CellPrimitive) must be the same as its encapsulated Type");
    };
}

unsafe impl<M: MemoryAttributes, P: CellPrimitive> Writable for WriteonlyCell<M, P> { }
impl_cell_traits_for!(WriteonlyCell);

#[doc = docs::side_by_side_md!("CellAccessor")]
pub struct CellAccessor<'mem, C: Cell> {
    cell_ptr: *mut <C::Primitive as CellPrimitive>::Type,
    _memory_lifetime: PhantomData<&'mem C>
}

impl<'mem, C: Cell> CellAccessor<'mem, C> {
    #[doc = docs::side_by_side_md!("CellAccessor.new")]
    pub const unsafe fn new(cell_ptr: *mut C) -> Self where C: Cell {
        Self {
            cell_ptr: cell_ptr as *mut <C::Primitive as CellPrimitive>::Type,
            _memory_lifetime: PhantomData
        }
    }

    #[doc = docs::side_by_side_md!("CellAccessor.get")]
    pub const unsafe fn get(&self) -> *mut <C::Primitive as CellPrimitive>::Type { self.cell_ptr }
}

pub mod prelude {
    pub use super::{
        CellAccessor,
        ReadonlyCell,
        ReadWriteCell,
        WriteonlyCell
    };
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use crate::test_doubles::Dummy;

    use super::*;

    #[test]
    fn cell_ptr__get__expect_same_value_passed_to_constructor() {
        let mut cell = Dummy;
        let accessor = unsafe { CellAccessor::new(&raw mut cell) };
        expect!(accessor.cell_ptr).to_equal(&raw mut cell as *mut usize);
    }

    #[test]
    fn get__called__expect_same_value_passed_to_constructor() {
        let mut cell = Dummy;
        let accessor = unsafe { CellAccessor::new(&raw mut cell) };
        expect!(unsafe { accessor.get() }).to_equal(&raw mut cell as *mut usize);
    }
}
