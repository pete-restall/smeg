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
}
