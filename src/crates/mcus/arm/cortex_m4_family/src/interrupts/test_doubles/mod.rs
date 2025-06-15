#![doc = smeg_kernel::docs::side_by_side_md!()]
use smeg_kernel::docs;

pub mod isr_context;

pub mod isr_stack_frames;

pub mod isr_vectors;

#[derive(Copy, Clone, Debug, PartialEq)]
#[doc = docs::side_by_side_md!("Dummy")]
pub struct Dummy;

#[derive(Copy, Clone, Debug, PartialEq)]
#[doc = docs::side_by_side_md!("Stub")]
pub struct Stub;
