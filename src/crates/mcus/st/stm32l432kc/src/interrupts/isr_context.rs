use core::convert::AsMut;

pub type FamilyIsrContext = smeg_mcu_arm_cortex_m4_family::interrupts::IsrContext;

pub struct IsrContext {
    cortex_m4: FamilyIsrContext
}

impl smeg_kernel::interrupts::IsrContext for IsrContext { }

impl From<FamilyIsrContext> for IsrContext {
    fn from(value: FamilyIsrContext) -> Self {
        Self { cortex_m4: value }
    }
}

impl AsRef<FamilyIsrContext> for IsrContext {
    fn as_ref(&self) -> &FamilyIsrContext { &self.cortex_m4 }
}

impl AsMut<FamilyIsrContext> for IsrContext {
    fn as_mut(&mut self) -> &mut FamilyIsrContext { &mut self.cortex_m4 }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use smeg_mcu_arm_cortex_m4_family::interrupts::{HasIsrBasicStackFrame, IsrBasicStackFrame};
    use smeg_mcu_arm_cortex_m4_family::interrupts::test_doubles::Dummy;

    use super::*;

    #[test]
    fn from__called_with_cortex_m4_isr_context__expect_cortex_m4_isr_context_is_moved() {
        let mut basic_stack_frame = IsrBasicStackFrame::from(Dummy);
        let cortex_m4_isr_context = FamilyIsrContext::from(&raw mut basic_stack_frame);
        let isr_context = IsrContext::from(cortex_m4_isr_context);
        expect!(unsafe { &raw const *isr_context.cortex_m4.basic_stack_frame() }).to_equal(&raw const basic_stack_frame);
    }

    #[test]
    fn as_ref__called_for_immutable_cortex_m4_isr_context__expect_cortex_m4_isr_context_is_referenced() {
        let mut basic_stack_frame = IsrBasicStackFrame::from(Dummy);
        let cortex_m4_isr_context = FamilyIsrContext::from(&raw mut basic_stack_frame);
        let isr_context = IsrContext::from(cortex_m4_isr_context);
        let context_as_ref = isr_context.as_ref();
        expect!(&raw const *context_as_ref).to_equal(&raw const isr_context.cortex_m4);
    }

    #[test]
    fn as_mut__called_for_mutable_cortex_m4_isr_context__expect_cortex_m4_isr_context_is_referenced() {
        let mut basic_stack_frame = IsrBasicStackFrame::from(Dummy);
        let cortex_m4_isr_context = FamilyIsrContext::from(&raw mut basic_stack_frame);
        let mut isr_context = IsrContext::from(cortex_m4_isr_context);
        let context_as_mut = isr_context.as_mut();
        expect!(&raw mut *context_as_mut).to_equal(&raw mut isr_context.cortex_m4);
    }
}
