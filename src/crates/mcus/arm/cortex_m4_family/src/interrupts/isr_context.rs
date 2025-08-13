use crate::interrupts::{HasIsrBasicStackFrame, HasIsrBasicStackFrameMut, IsrBasicStackFrame};

pub struct IsrContext {
    basic_stack_frame: *mut IsrBasicStackFrame
}

impl smeg_kernel::interrupts::IsrContext for IsrContext { }

impl From<*mut IsrBasicStackFrame> for IsrContext {
    fn from(value: *mut IsrBasicStackFrame) -> Self {
        Self { basic_stack_frame: value }
    }
}

unsafe impl HasIsrBasicStackFrame for IsrContext {
    unsafe fn basic_stack_frame(&self) -> &IsrBasicStackFrame {
        unsafe { &*self.basic_stack_frame }
    }
}

unsafe impl HasIsrBasicStackFrameMut for IsrContext {
    unsafe fn basic_stack_frame_mut(&mut self) -> &mut IsrBasicStackFrame {
        unsafe { &mut *self.basic_stack_frame }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use crate::interrupts::test_doubles::Dummy;

    use super::*;

    #[test]
    fn basic_stack_frame__get_after_from__expect_same_value() {
        let mut basic_stack_frame = IsrBasicStackFrame::from(Dummy);
        let isr_context = IsrContext::from(&raw mut basic_stack_frame);
        expect!(isr_context.basic_stack_frame).to_equal(&raw mut basic_stack_frame);
    }

    #[test]
    fn basic_stack_frame__called__expect_dereferenced_value_of_pointer_from_construction() {
        let mut basic_stack_frame = IsrBasicStackFrame::from(Dummy);
        let isr_context = IsrContext { basic_stack_frame: &raw mut basic_stack_frame };
        let dereferenced_basic_stack_frame = unsafe { isr_context.basic_stack_frame() };
        expect!(&raw const *dereferenced_basic_stack_frame).to_equal(isr_context.basic_stack_frame);
    }

    #[test]
    fn basic_stack_frame_mut__called__expect_dereferenced_value_of_pointer_from_construction() {
        let mut basic_stack_frame = IsrBasicStackFrame::from(Dummy);
        let mut isr_context = IsrContext { basic_stack_frame: &raw mut basic_stack_frame };
        let dereferenced_basic_stack_frame = unsafe { isr_context.basic_stack_frame_mut() };
        expect!(&raw mut *dereferenced_basic_stack_frame).to_equal(isr_context.basic_stack_frame);
    }
}
