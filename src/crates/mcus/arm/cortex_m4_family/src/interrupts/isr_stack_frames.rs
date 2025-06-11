use smeg_kernel::docs;

#[repr(C)]
#[doc = docs::side_by_side_md!("IsrBasicStackFrame")]
pub struct IsrBasicStackFrame {
    #[doc = docs::side_by_side_md!("IsrBasicStackFrame.r0")]
    pub r0: usize,

    #[doc = docs::side_by_side_md!("IsrBasicStackFrame.r1")]
    pub r1: usize,

    #[doc = docs::side_by_side_md!("IsrBasicStackFrame.r2")]
    pub r2: usize,

    #[doc = docs::side_by_side_md!("IsrBasicStackFrame.r3")]
    pub r3: usize,

    #[doc = docs::side_by_side_md!("IsrBasicStackFrame.r12")]
    pub r12: usize,

    #[doc = docs::side_by_side_md!("IsrBasicStackFrame.r14_lr")]
    pub r14_lr: usize,

    #[doc = docs::side_by_side_md!("IsrBasicStackFrame.return_address")]
    pub return_address: usize,

    #[doc = docs::side_by_side_md!("IsrBasicStackFrame.xpsr")]
    pub xpsr: usize
}

const _: () = {
    assert!(size_of::<IsrBasicStackFrame>() == 8 * size_of::<usize>(), "Size of IsrBasicStackFrame must be exactly 8 machine words");
    assert!(align_of::<IsrBasicStackFrame>() == align_of::<usize>(), "Alignment of IsrBasicStackFrame must be the same as a machine word");
};

pub mod prelude {
    pub use super::IsrBasicStackFrame;
}
