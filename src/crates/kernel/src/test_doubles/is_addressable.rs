#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

use crate::{IsAddressable, IsAddressableMut};

use super::Dummy;

impl<T> IsAddressable<T> for Dummy {
    #[doc = docs::side_by_side_md!("Dummy.is_addressable")]
    fn is_addressable(&self, _ptr: *const T) -> bool { false }
}

impl<T> IsAddressableMut<T> for Dummy {
    #[doc = docs::side_by_side_md!("Dummy.is_addressable_mut")]
    fn is_addressable_mut(&self, _ptr: *mut T) -> bool { false }
}
