#[doc = crate::docs::side_by_side_md!()]
use crate::docs;

#[doc = docs::side_by_side_md!("MemoryAttributes")]
pub unsafe trait MemoryAttributes { }

#[doc = docs::side_by_side_md!("Addressable")]
pub unsafe trait Addressable { type MemoryAttributes: MemoryAttributes; }

#[doc = docs::side_by_side_md!("Bank")]
pub unsafe trait Bank: Addressable { }

#[doc = docs::side_by_side_md!("Cell")]
pub unsafe trait Cell: Addressable { type Type: Copy; }

#[doc = docs::side_by_side_md!("Readable")]
pub unsafe trait Readable { }

#[doc = docs::side_by_side_md!("Writable")]
pub unsafe trait Writable { }

mod banks;
pub use banks::prelude::*;

mod cells;
pub use cells::prelude::*;

#[cfg(feature = "test_doubles")]
pub mod test_doubles {
    use crate::test_doubles::Dummy;

    use super::*;

    unsafe impl MemoryAttributes for Dummy { }

    unsafe impl Addressable for Dummy { type MemoryAttributes = Dummy; }

    unsafe impl Bank for Dummy { }

    unsafe impl Cell for Dummy { type Type = usize; }
}
