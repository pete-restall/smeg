use core::borrow::{Borrow, BorrowMut};

use smeg_kernel::McuSingleCore;

use smeg_mcu_arm_cortex_m4_family::interrupts as cortex_m4_interrupts;

pub struct IsrContextImpl { // TODO: can rename this (and the M4 one) to IsrContext I think, now that there's no longer the trait...
    cortex_m4: cortex_m4_interrupts::IsrContextImpl
}

impl smeg_kernel::interrupts::IsrContext for IsrContextImpl {
    type Mcu = McuSingleCore;
}

impl From<cortex_m4_interrupts::IsrContextImpl> for IsrContextImpl {
    fn from(value: cortex_m4_interrupts::IsrContextImpl) -> Self {
        Self { cortex_m4: value }
    }
}

impl Borrow<cortex_m4_interrupts::IsrContextImpl> for IsrContextImpl {
    fn borrow(&self) -> &cortex_m4_interrupts::IsrContextImpl {
        &self.cortex_m4
    }
}

impl BorrowMut<cortex_m4_interrupts::IsrContextImpl> for IsrContextImpl {
    fn borrow_mut(&mut self) -> &mut cortex_m4_interrupts::IsrContextImpl {
        &mut self.cortex_m4
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use smeg_kernel::HasMcuCoreId;
    use smeg_kernel::interrupts::IsrContext;

    use cortex_m4_interrupts::{HasIsrBasicStackFrame, IsrBasicStackFrame};

    use cortex_m4_interrupts::test_doubles::Dummy;

    use super::*;

    #[test]
    fn mcu__get_core_id__expect_hard_coded_zero() {
        expect!(<IsrContextImpl as IsrContext>::Mcu::core_id()).to_equal(0);
    }

    #[test]
    fn from__called_with_cortex_m4_isr_context__expect_cortex_m4_isr_context_is_moved() {
        let mut basic_stack_frame = IsrBasicStackFrame::from(Dummy);
        let cortex_m4_isr_context = cortex_m4_interrupts::IsrContextImpl::from(&raw mut basic_stack_frame);
        let isr_context = IsrContextImpl::from(cortex_m4_isr_context);
        expect!(unsafe { &raw const *isr_context.cortex_m4.basic_stack_frame() }).to_equal(&raw const basic_stack_frame);
    }

    #[test]
    fn borrow__called_for_cortex_m4_isr_context__expect_cortex_m4_isr_context_is_borrowed() {
        let mut basic_stack_frame = IsrBasicStackFrame::from(Dummy);
        let cortex_m4_isr_context = cortex_m4_interrupts::IsrContextImpl::from(&raw mut basic_stack_frame);
        let isr_context = IsrContextImpl::from(cortex_m4_isr_context);
        let borrowed_context: &cortex_m4_interrupts::IsrContextImpl = isr_context.borrow();
        expect!(&raw const *borrowed_context).to_equal(&raw const isr_context.cortex_m4);
    }

    #[test]
    fn borrow_mut__called_for_cortex_m4_isr_context__expect_cortex_m4_isr_context_is_borrowed() {
        let mut basic_stack_frame = IsrBasicStackFrame::from(Dummy);
        let cortex_m4_isr_context = cortex_m4_interrupts::IsrContextImpl::from(&raw mut basic_stack_frame);
        let mut isr_context = IsrContextImpl::from(cortex_m4_isr_context);
        let borrowed_context: &mut cortex_m4_interrupts::IsrContextImpl = isr_context.borrow_mut();
        expect!(&raw mut *borrowed_context).to_equal(&raw mut isr_context.cortex_m4);
    }
}
