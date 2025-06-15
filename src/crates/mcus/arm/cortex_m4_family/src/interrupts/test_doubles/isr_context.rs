#![doc = smeg_kernel::docs::side_by_side_md!()]
use smeg_kernel::docs;

use core::borrow::{Borrow, BorrowMut};

use crate::interrupts::{HasIsrBasicStackFrame, HasIsrBasicStackFrameMut, IsrBasicStackFrame, IsrContextImpl};

use super::Dummy;

impl smeg_kernel::interrupts::IsrContext for Dummy {
    type Mcu = smeg_kernel::test_doubles::Dummy;
}

impl From<*mut IsrBasicStackFrame> for Dummy {
    #[doc = docs::side_by_side_md!("From<*mut IsrBasicStackFrame>.from")]
    fn from(_value: *mut IsrBasicStackFrame) -> Self {
        panic!("Aborting because From::<*mut IsrBasicStackFrame>::from() -> Dummy should never be called");
    }
}

impl From<IsrContextImpl> for Dummy {
    #[doc = docs::side_by_side_md!("From<IsrContextImpl>.from")]
    fn from(_value: IsrContextImpl) -> Self {
        panic!("Aborting because From::<IsrContextImpl>::from() -> Dummy should never be called");
    }
}

unsafe impl HasIsrBasicStackFrame for Dummy {
    #[doc = docs::side_by_side_md!("HasIsrBasicStackFrame.basic_stack_frame")]
    unsafe fn basic_stack_frame(&self) -> &IsrBasicStackFrame {
        panic!("Aborting because HasIsrBasicStackFrame::basic_stack_frame(&Dummy) should never be called");
    }
}

unsafe impl HasIsrBasicStackFrameMut for Dummy {
    #[doc = docs::side_by_side_md!("HasIsrBasicStackFrameMut.basic_stack_frame_mut")]
    unsafe fn basic_stack_frame_mut(&mut self) -> &mut IsrBasicStackFrame {
        panic!("Aborting because HasIsrBasicStackFrameMut::basic_stack_frame_mut(&mut Dummy) should never be called");
    }
}

impl Borrow<IsrContextImpl> for Dummy {
    #[doc = docs::side_by_side_md!("Borrow.borrow")]
    fn borrow(&self) -> &IsrContextImpl {
        panic!("Aborting because Borrow::<IsrContextImpl>::borrow(&Dummy) should never be called");
    }
}

impl BorrowMut<IsrContextImpl> for Dummy {
    #[doc = docs::side_by_side_md!("BorrowMut.borrow_mut")]
    fn borrow_mut(&mut self) -> &mut IsrContextImpl {
        panic!("Aborting because BorrowMut::<IsrContextImpl>::borrow_mut(&mut Dummy) should never be called");
    }
}
