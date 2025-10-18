use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

use crate::docs;

use super::{Addressable, Cell, MemoryAttributes, Readable, Writable};

macro_rules! impl_cell_traits_for {
    ($name:ident) => {
        unsafe impl<M: MemoryAttributes, T: Copy> Cell for $name<M, T> { type Type = T; }
        unsafe impl<M: MemoryAttributes, T: Copy> Addressable for $name<M, T> { type MemoryAttributes = M; }
        impl<M, T> !Send for $name<M, T> { }
        impl<M, T> !Sync for $name<M, T> { }
        impl<M, T> !Deref for $name<M, T> { }
        impl<M, T> !DerefMut for $name<M, T> { }
    };
}

#[repr(transparent)]
#[doc = docs::side_by_side_md!("ReadWriteCell")]
pub struct ReadWriteCell<M: MemoryAttributes, T: Copy> {
    value: T,
    _attributes: PhantomData<M>
}

impl<M: MemoryAttributes, T: Copy> ReadWriteCell<M, T> {
    const _ENSURE_LAYOUT_OF_UNDERLYING_TYPE: () = {
        assert!(size_of::<ReadWriteCell<M, T>>() == size_of::<T>(), "Size of ReadWriteCell<M, T> must be the same size as T");
        assert!(align_of::<ReadWriteCell<M, T>>() == align_of::<T>(), "Alignment of ReadWriteCell<M, T> must be the same as T");
    };
}

unsafe impl<M: MemoryAttributes, T: Copy> Readable for ReadWriteCell<M, T> { }
unsafe impl<M: MemoryAttributes, T: Copy> Writable for ReadWriteCell<M, T> { }
impl_cell_traits_for!(ReadWriteCell);

#[repr(transparent)]
#[doc = docs::side_by_side_md!("ReadonlyCell")]
pub struct ReadonlyCell<M: MemoryAttributes, T: Copy> {
    value: T,
    _attributes: PhantomData<M>
}

impl<M: MemoryAttributes, T: Copy> ReadonlyCell<M, T> {
    const _ENSURE_LAYOUT_OF_UNDERLYING_TYPE: () = {
        assert!(size_of::<ReadonlyCell<M, T>>() == size_of::<T>(), "Size of ReadonlyCell<M, T> must be the same size as T");
        assert!(align_of::<ReadonlyCell<M, T>>() == align_of::<T>(), "Alignment of ReadonlyCell<M, T> must be the same as T");
    };
}

unsafe impl<M: MemoryAttributes, T: Copy> Readable for ReadonlyCell<M, T> { }
impl_cell_traits_for!(ReadonlyCell);

#[repr(transparent)]
#[doc = docs::side_by_side_md!("WriteonlyCell")]
pub struct WriteonlyCell<M: MemoryAttributes, T: Copy> {
    value: T,
    _attributes: PhantomData<M>
}

impl<M: MemoryAttributes, T: Copy> WriteonlyCell<M, T> {
    const _ENSURE_LAYOUT_OF_UNDERLYING_TYPE: () = {
        assert!(size_of::<WriteonlyCell<M, T>>() == size_of::<T>(), "Size of WriteonlyCell<M, T> must be the same size as T");
        assert!(align_of::<WriteonlyCell<M, T>>() == align_of::<T>(), "Alignment of WriteonlyCell<M, T> must be the same as T");
    };
}

unsafe impl<M: MemoryAttributes, T: Copy> Writable for WriteonlyCell<M, T> { }
impl_cell_traits_for!(WriteonlyCell);

#[doc = docs::side_by_side_md!("CellAccessor")]
pub struct CellAccessor<'mem, C: Cell> {
    cell_ptr: *mut C,
    _memory_lifetime: PhantomData<&'mem C>
}

impl<'mem, C: Cell> CellAccessor<'mem, C> {
    #[doc = docs::side_by_side_md!("CellAccessor.new")]
    pub const unsafe fn new(cell_ptr: *mut C) -> Self {
        Self { cell_ptr, _memory_lifetime: PhantomData }
    }

    #[doc = docs::side_by_side_md!("CellAccessor.get")]
    pub const unsafe fn get(&self) -> *mut C::Type { self.cell_ptr as *mut C::Type }
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
        expect!(accessor.cell_ptr).to_equal(&raw mut cell);
    }

    #[test]
    fn get__called__expect_same_value_passed_to_constructor() {
        let mut cell = Dummy;
        let accessor = unsafe { CellAccessor::new(&raw mut cell) };
        expect!(unsafe { accessor.get() }).to_equal(&raw mut cell as *mut usize);
    }
}
