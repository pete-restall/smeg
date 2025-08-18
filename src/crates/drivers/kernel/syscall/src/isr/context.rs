use core::mem::MaybeUninit;

use smeg_kernel::interrupts::IsrContext;

use crate::SyscallArgs;

pub struct SyscallIsrContext<'isr, I: IsrContext, A: SyscallArgs> {
    isr_context: &'isr mut I,
    args: &'isr mut MaybeUninit<A>
}

impl<'isr, I: IsrContext, A: SyscallArgs> SyscallIsrContext<'isr, I, A> {
    pub(crate) fn new(isr_context: &'isr mut I, args: &'isr mut MaybeUninit<A>) -> Self {
        Self { isr_context, args }
    }

    pub fn isr_mut(&mut self) -> &mut I {
        self.isr_context
    }

    pub fn unvalidated_args_mut(&mut self) -> &mut MaybeUninit<A> {
        self.args
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use smeg_kernel::test_doubles::Dummy;

    use super::*;

    #[test]
    fn new__called__expect_same_isr_context() {
        let mut isr_context = Dummy;
        let mut args = MaybeUninit::<Dummy>::uninit();
        let syscall_isr_context = SyscallIsrContext::new(&mut isr_context, &mut args);
        expect!(&raw mut *syscall_isr_context.isr_context).to_equal(&raw mut isr_context);
    }

    #[test]
    fn new__called__expect_same_args() {
        let mut isr_context = Dummy;
        let mut args = MaybeUninit::<Dummy>::uninit();
        let syscall_isr_context = SyscallIsrContext::new(&mut isr_context, &mut args);
        expect!(&raw mut *syscall_isr_context.args).to_equal(&raw mut args);
    }

    #[test]
    fn isr_mut__called__expect_same_isr_context_passed_to_constructor() {
        let mut isr_context = Dummy;
        let mut args = MaybeUninit::<Dummy>::uninit();
        let mut syscall_isr_context = SyscallIsrContext::new(&mut isr_context, &mut args);
        let from_isr_mut = syscall_isr_context.isr_mut();
        expect!(&raw mut *from_isr_mut).to_equal(&raw mut isr_context);
    }

    #[test]
    fn unvalidated_args_mut__called__expect_same_args_passed_to_constructor() {
        let mut isr_context = Dummy;
        let mut args = MaybeUninit::<Dummy>::uninit();
        let mut syscall_isr_context = SyscallIsrContext::new(&mut isr_context, &mut args);
        let unvalidated_args_mut = syscall_isr_context.unvalidated_args_mut();
        expect!(&raw mut *unvalidated_args_mut).to_equal(&raw mut args);
    }
}
