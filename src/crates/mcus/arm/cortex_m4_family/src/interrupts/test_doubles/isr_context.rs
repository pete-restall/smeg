#![doc = smeg_kernel::docs::side_by_side_md!()]
use smeg_kernel::docs;

use std::boxed::Box;
use std::convert::{AsMut, AsRef};

use smeg_kernel::test_doubles::StubFor;

use crate::interrupts::{HasIsrBasicStackFrame, HasIsrBasicStackFrameMut, IsrBasicStackFrame, IsrContext};

use super::{Dummy, Stub};

impl smeg_kernel::interrupts::IsrContext for Dummy { }

impl From<*mut IsrBasicStackFrame> for Dummy {
    #[doc = docs::side_by_side_md!("From<*mut IsrBasicStackFrame>.from")]
    fn from(_value: *mut IsrBasicStackFrame) -> Self {
        panic!("Aborting because From::<*mut IsrBasicStackFrame>::from() -> Dummy should never be called");
    }
}

impl From<IsrContext> for Dummy {
    #[doc = docs::side_by_side_md!("From<IsrContextImpl>.from")]
    fn from(_value: IsrContext) -> Self {
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

impl AsRef<IsrContext> for Dummy {
    #[doc = docs::side_by_side_md!("AsRef.as_ref")]
    fn as_ref(&self) -> &IsrContext {
        panic!("Aborting because AsRef::<IsrContextImpl>::as_ref(&Dummy) should never be called");
    }
}

impl AsMut<IsrContext> for Dummy {
    #[doc = docs::side_by_side_md!("AsMut.as_mut")]
    fn as_mut(&mut self) -> &mut IsrContext {
        panic!("Aborting because AsMut::<IsrContextImpl>::as_mut(&Dummy) should never be called");
    }
}

pub struct StubIsrContext<T> {
    _stack_frame_for_ptr: Box<IsrBasicStackFrame>,
    cortex_m4: IsrContext,
    pub stubbed_with: Option<T>
}

impl<T> From<StubFor<T>> for StubIsrContext<T> {
    fn from(stub: StubFor<T>) -> Self {
        let mut stack_frame = Box::new(IsrBasicStackFrame::from(Stub));
        let stack_frame_ptr = stack_frame.as_mut() as *mut IsrBasicStackFrame;
        Self {
            _stack_frame_for_ptr: stack_frame,
            cortex_m4: IsrContext::from(stack_frame_ptr),
            stubbed_with: Some(stub.value)
        }
    }
}

impl<T> From<IsrContext> for StubIsrContext<T> {
    fn from(stub: IsrContext) -> Self {
        let stack_frame = Box::new(IsrBasicStackFrame::from(Stub));
        Self {
            _stack_frame_for_ptr: stack_frame,
            cortex_m4: stub,
            stubbed_with: None
        }
    }
}

impl<T> smeg_kernel::interrupts::IsrContext for StubIsrContext<T> { }

impl<T> AsRef<IsrContext> for StubIsrContext<T> {
    fn as_ref(&self) -> &IsrContext { &self.cortex_m4 }
}

impl<T> AsMut<IsrContext> for StubIsrContext<T> {
    fn as_mut(&mut self) -> &mut IsrContext { &mut self.cortex_m4 }
}
