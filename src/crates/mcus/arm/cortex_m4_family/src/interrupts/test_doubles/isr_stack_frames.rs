#![doc = smeg_kernel::docs::side_by_side_md!()]
use smeg_kernel::docs;

use crate::interrupts::IsrBasicStackFrame;

use smeg_testing_host_utils::integers::any_usize;

use super::{Dummy, Stub};

impl From<Dummy> for IsrBasicStackFrame {
    #[doc = docs::side_by_side_md!("From<Dummy>.from")]
    fn from(_value: Dummy) -> Self {
        Self::from(Stub)
    }
}

impl From<Stub> for IsrBasicStackFrame {
    #[doc = docs::side_by_side_md!("From<Stub>.from")]
    fn from(_value: Stub) -> Self {
        Self {
            r0: any_usize(),
            r1: any_usize(),
            r2: any_usize(),
            r3: any_usize(),
            r12: any_usize(),
            r14_lr: any_usize(),
            return_address: any_usize(),
            xpsr: any_usize()
        }
    }
}
