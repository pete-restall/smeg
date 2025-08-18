#![doc = crate::docs::side_by_side_md!()]
use crate::docs;

pub mod has_mcu_core_id;
pub mod is_addressable;

#[derive(Copy, Clone, Debug, PartialEq)]
#[doc = docs::side_by_side_md!("Dummy")]
pub struct Dummy;

#[derive(Copy, Clone, Debug, PartialEq)]
#[doc = docs::side_by_side_md!("Stub")]
pub struct Stub;

#[derive(Copy, Clone, Debug, PartialEq)]
#[doc = docs::side_by_side_md!("StubFor")]
pub struct StubFor<T> {
    pub value: T
}
