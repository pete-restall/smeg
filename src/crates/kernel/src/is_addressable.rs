use crate::docs;

#[doc = docs::side_by_side_md!("IsAddressable")]
pub trait IsAddressable<T> {
    #[doc = docs::side_by_side_md!("IsAddressable.is_addressable")]
    fn is_addressable(&self, ptr: *const T) -> bool;
}

#[doc = docs::side_by_side_md!("IsAddressableMut")]
pub trait IsAddressableMut<T> {
    #[doc = docs::side_by_side_md!("IsAddressableMut.is_addressable_mut")]
    fn is_addressable_mut(&self, ptr: *mut T) -> bool;
}
