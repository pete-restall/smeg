use core::convert::AsMut;
use core::mem::MaybeUninit;

use smeg_kernel::errors::{error_tag, KernelError, KernelErrorCode, TaggedError};
use smeg_kernel::IsAddressableMut;
use smeg_kernel::interrupts::IsrContext;
use smeg_kernel::tasks::{HasInterruptedTask, HasTaskScheduler};

use crate::SyscallResult;

use super::{SyscallIsrContext, SyscallIsrHandler};

pub type SyscallIsrTrampolinePtr<I: IsrContext> = unsafe fn(isr_context: &mut I, args: usize) -> SyscallResult;

pub unsafe trait SyscallIsrTrampoline<I: IsrContext> {
    unsafe fn on_syscall(isr_context: &mut I, args: usize) -> SyscallResult;
}

unsafe impl<H: SyscallIsrHandler> SyscallIsrTrampoline<H::IsrContext> for H where
    H::IsrContext: IsrContext + HasTaskScheduler + AsMut<<H::IsrContext as HasTaskScheduler>::TaskScheduler>,
    <H::IsrContext as HasTaskScheduler>::TaskScheduler: HasInterruptedTask,
    <<H::IsrContext as HasTaskScheduler>::TaskScheduler as HasInterruptedTask>::InterruptedTask: IsAddressableMut<MaybeUninit<H::Args>> {

    unsafe fn on_syscall(isr_context: &mut H::IsrContext, args: usize) -> SyscallResult {
        let args_ptr = args as *mut MaybeUninit<H::Args>;
        let task_scheduler = isr_context.as_mut();
        if let Some(interrupted_task) = task_scheduler.interrupted_task()  {
            if !interrupted_task.is_addressable_mut(args_ptr) {
                return Err(KernelError::from(TaggedError::new(
                    KernelErrorCode::UnaddressableSyscallArgs,
                    error_tag!("Syscall Arguments are not addressable by the interrupted task (caller)"))));
            }
        } else if (align_of::<H::Args>() > 1) && (args & (align_of::<H::Args>() - 1)) != 0 {
            return Err(KernelError::from(TaggedError::new(KernelErrorCode::UnalignedSyscallArgs, error_tag!("Syscall Arguments are unaligned"))));
        }

        let mut context = SyscallIsrContext::new(isr_context, unsafe { &mut *args_ptr });
        <H as SyscallIsrHandler>::on_syscall(&mut context)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use core::cell::Cell;
    use core::convert::AsMut;
    use core::marker::PhantomData;
    use core::num::NonZero;

    use smeg_kernel::IsAddressableMut;
    use smeg_kernel::errors::{KernelErrorCode, ResultToUsizeResultConversion, UsizeResultConversions};
    use smeg_kernel::interrupts::IsrContext;
    use smeg_kernel::tasks::{HasInterruptedTask, HasTaskScheduler, Task, TaskScheduler};
    use smeg_kernel::test_doubles::Dummy;

    use fluent_test::prelude::*;

    use crate::{HasSyscallId, SyscallArgs, SyscallResult};

    use super::*;
    use super::super::{SyscallIsrContext, SyscallIsrHandler};

    struct ZeroSizedArgs;
    const _: () = assert!(size_of::<ZeroSizedArgs>() == 0, "Size of ZeroSizedArgs must be 0");

    impl HasSyscallId for ZeroSizedArgs {
        fn syscall_id() -> usize { 0 }
    }

    #[repr(C, align(2))]
    #[derive(Copy, Clone)]
    struct Aligned2Args;
    const _: () = assert!(align_of::<Aligned2Args>() == 2, "Alignment of Aligned2Args must be 2");

    impl HasSyscallId for Aligned2Args {
        fn syscall_id() -> usize { 0 }
    }

    #[repr(C, align(4))]
    #[derive(Copy, Clone)]
    struct Aligned4Args;
    const _: () = assert!(align_of::<Aligned4Args>() == 4, "Alignment of Aligned4Args must be 4");

    impl HasSyscallId for Aligned4Args {
        fn syscall_id() -> usize { 0 }
    }

    #[repr(C, align(64))]
    #[derive(Copy, Clone)]
    struct Aligned64Args;
    const _: () = assert!(align_of::<Aligned64Args>() == 64, "Alignment of Aligned64Args must be 64");

    impl HasSyscallId for Aligned64Args {
        fn syscall_id() -> usize { 0 }
    }

    struct StubInterruptedTask {
        ptr: Option<usize>,
        is_addressable: bool
    }

    impl StubInterruptedTask {
        pub fn stub_is_addressable_for<T>(ptr: *mut T, is_addressable: bool) -> Self {
            Self { ptr: Some(ptr as usize), is_addressable }
        }

        pub fn stub_is_addressable(is_addressable: bool) -> Self {
            Self { ptr: None, is_addressable }
        }
    }

    impl Task for StubInterruptedTask { }

    impl<T> IsAddressableMut<T> for StubInterruptedTask {
        fn is_addressable_mut(&self, ptr: *mut T) -> bool {
            let ptr = ptr as usize;
            if let Some(expected_ptr) = self.ptr {
                (ptr == expected_ptr && self.is_addressable) || (ptr != expected_ptr && !self.is_addressable)
            } else {
                self.is_addressable
            }
        }
    }

    struct StubTaskScheduler<T: Task> {
        interrupted_task: Option<T>
    }

    impl<T: Task> TaskScheduler for StubTaskScheduler<T> { }

    impl<T: Task> StubTaskScheduler<T> {
        pub fn stub_with_no_interrupted_task() -> Self {
            Self { interrupted_task: None }
        }

        pub fn stub_with_interrupted_task(interrupted_task: T) -> Self {
            Self { interrupted_task: Some(interrupted_task) }
        }
    }

    impl<T: Task> HasInterruptedTask for StubTaskScheduler<T> {
        type InterruptedTask = T;

        fn interrupted_task(&self) -> Option<&Self::InterruptedTask> {
            if let Some(interrupted_task) = &self.interrupted_task {
                Some(interrupted_task)
            } else {
                None
            }
        }
    }

    struct StubIsrContext<S: TaskScheduler> {
        task_scheduler: S
    }

    impl<S: TaskScheduler> StubIsrContext<S> {
        pub fn stub_for_task_scheduler(task_scheduler: S) -> Self {
            Self { task_scheduler }
        }
    }

    impl<S: TaskScheduler> IsrContext for StubIsrContext<S> { }

    impl<S: TaskScheduler> AsMut<S> for StubIsrContext<S> {
        fn as_mut(&mut self) -> &mut S { &mut self.task_scheduler }
    }

    impl<S: TaskScheduler> HasTaskScheduler for StubIsrContext<S> {
        type TaskScheduler = S;
    }

    struct StubUnimplementedSyscallHandler<I: IsrContext, A: HasSyscallId> {
        _isr_context: PhantomData<I>,
        _args: PhantomData<A>
    }

    impl<I: IsrContext, A: HasSyscallId> SyscallIsrHandler for StubUnimplementedSyscallHandler<I, A> {
        type IsrContext = I;
        type Args = A;

        fn on_syscall(_context: &mut SyscallIsrContext<Self::IsrContext, Self::Args>) -> SyscallResult {
            panic!("on_syscall() should never be called");
        }
    }

    struct StubSyscallHandler<I: IsrContext, A: HasSyscallId> {
        _isr_context: PhantomData<I>,
        _args: PhantomData<A>
    }

    impl<I: IsrContext, A: HasSyscallId> StubSyscallHandler<I, A> {
        thread_local! {
            static TLS: Cell<(usize, usize, SyscallResult)> = panic!("StubSyscallHandler TLS has not been initialised");
        }

        pub fn stub_for(isr_context: &I, args: &mut A, result: SyscallResult) -> Self {
            Self::stub_for_ptr(isr_context, &raw mut *args , result)
        }

        pub fn stub_for_ptr(isr_context: &I, args: *mut A, result: SyscallResult) -> Self {
            Self::TLS.set((&raw const *isr_context as usize, args as usize, result));
            Self {
                _isr_context: PhantomData,
                _args: PhantomData
            }
        }
    }

    impl<I: IsrContext, A: HasSyscallId> SyscallIsrHandler for StubSyscallHandler<I, A> {
        type IsrContext = I;
        type Args = A;

        fn on_syscall(context: &mut SyscallIsrContext<Self::IsrContext, Self::Args>) -> SyscallResult {
            let (expected_isr_context_addr, expected_args_addr, result) = Self::TLS.get();

            let isr_context = context.isr_mut();
            let isr_context_addr = &raw mut *isr_context as usize;
            let args = context.unvalidated_args_mut();
            let args_addr = &raw mut *args as usize;

            if isr_context_addr != expected_isr_context_addr || args_addr != expected_args_addr {
                panic!("on_syscall() has not been stubbed for the given arguments");
            }

            result
        }
    }

    #[test]
    fn on_syscall__called_when_args_is_not_addressable_by_interrupted_task__expect_unaddressable_syscall_args_err_is_returned() {
        type SyscallHandler = StubUnimplementedSyscallHandler<StubIsrContext<StubTaskScheduler<StubInterruptedTask>>, Dummy>;

        let mut isr_context = StubIsrContext::stub_for_task_scheduler(
            StubTaskScheduler::stub_with_interrupted_task(
                StubInterruptedTask::stub_is_addressable(false)));

        let args = Dummy;
        let result = unsafe {
            <SyscallHandler as SyscallIsrTrampoline<_>>::on_syscall(&mut isr_context, &raw const args as usize)
        };

        expect!(result.unwrap_err().code).to_equal(KernelErrorCode::UnaddressableSyscallArgs);
    }

    #[test]
    fn on_syscall__called_when_no_interrupted_task_and_unaligned_args__expect_unaligned_syscall_args_err_is_returned() {
        let mut args = [Aligned2Args; 3];
        _on_syscall__called_when_no_interrupted_task_and_unaligned_args__expect_unaligned_syscall_args_err_is_returned(&mut args[1]);

        let mut args = [Aligned4Args; 3];
        _on_syscall__called_when_no_interrupted_task_and_unaligned_args__expect_unaligned_syscall_args_err_is_returned(&mut args[1]);

        let mut args = [Aligned64Args; 3];
        _on_syscall__called_when_no_interrupted_task_and_unaligned_args__expect_unaligned_syscall_args_err_is_returned(&mut args[1]);
    }

    fn _on_syscall__called_when_no_interrupted_task_and_unaligned_args__expect_unaligned_syscall_args_err_is_returned<A: HasSyscallId>(args: &mut A) {
        on_syscall__called_when_no_interrupted_task_and_unaligned_args__expect::<A, _>(|offset| {
            let mut isr_context = StubIsrContext::stub_for_task_scheduler(StubTaskScheduler::stub_with_no_interrupted_task());
            let result = unsafe {
                <StubUnimplementedSyscallHandler<StubIsrContext<StubTaskScheduler<StubInterruptedTask>>, A> as SyscallIsrTrampoline<_>>::on_syscall(
                    &mut isr_context,
                    ((&raw const *args as isize) + offset) as usize)
            };

            expect!(result.unwrap_err().code).to_equal(KernelErrorCode::UnalignedSyscallArgs);
        });
    }

    fn on_syscall__called_when_no_interrupted_task_and_unaligned_args__expect<A, F: Fn(isize)>(assertion: F) {
        let bad_alignment = align_of::<A>() as isize - 1;
        for offset in -bad_alignment..=bad_alignment {
            if offset != 0 {
                assertion(offset);
            }
        }
    }

    #[test]
    fn on_syscall__called_with_zst_args_when_no_interrupted_task_and_handler_returns_ok__expect_ok_is_returned() {
        let ok = Ok(());
        on_syscall__called_with_given_args_when_no_interrupted_task__expect_same_result_as_handler_is_returned(ZeroSizedArgs, ok);
    }

    fn on_syscall__called_with_given_args_when_no_interrupted_task__expect_same_result_as_handler_is_returned<A: SyscallArgs>(
        mut args: A,
        result: SyscallResult) {

        let error_as_usize = match result {
            Ok(()) => 0,
            Err(error) => NonZero::<usize>::from(error).get()
        };

        type StubbedSyscallHandler<T> = StubSyscallHandler<StubIsrContext<StubTaskScheduler<StubInterruptedTask>>, T>;

        let mut isr_context = StubIsrContext::stub_for_task_scheduler(StubTaskScheduler::<StubInterruptedTask>::stub_with_no_interrupted_task());
        let _syscall_handler = StubbedSyscallHandler::stub_for(&isr_context, &mut args, result);
        let result = unsafe {
            <StubbedSyscallHandler<A> as SyscallIsrTrampoline<_>>::on_syscall(&mut isr_context, &raw const args as usize)
        }.as_usize_result();

        expect!(result.as_usize()).to_equal(error_as_usize);
    }

    #[test]
    fn on_syscall__called_with_zst_args_when_no_interrupted_task_and_handler_returns_err__expect_same_err_is_returned() {
        let error = Err(smeg_kernel::errors::test_doubles::any_kernel_error());
        on_syscall__called_with_given_args_when_no_interrupted_task__expect_same_result_as_handler_is_returned(ZeroSizedArgs, error);
    }

    #[test]
    fn on_syscall__called_with_aligned_args_when_no_interrupted_task_and_handler_returns_ok__expect_ok_is_returned() {
        let ok = Ok(());
        on_syscall__called_with_given_args_when_no_interrupted_task__expect_same_result_as_handler_is_returned(Aligned2Args, ok);
        on_syscall__called_with_given_args_when_no_interrupted_task__expect_same_result_as_handler_is_returned(Aligned4Args, ok);
        on_syscall__called_with_given_args_when_no_interrupted_task__expect_same_result_as_handler_is_returned(Aligned64Args, ok);
    }

    #[test]
    fn on_syscall__called_with_aligned_args_when_no_interrupted_task_and_handler_returns_err__expect_same_err_is_returned() {
        let error = Err(smeg_kernel::errors::test_doubles::any_kernel_error());
        on_syscall__called_with_given_args_when_no_interrupted_task__expect_same_result_as_handler_is_returned(Aligned2Args, error);

        let error = Err(smeg_kernel::errors::test_doubles::any_kernel_error());
        on_syscall__called_with_given_args_when_no_interrupted_task__expect_same_result_as_handler_is_returned(Aligned4Args, error);

        let error = Err(smeg_kernel::errors::test_doubles::any_kernel_error());
        on_syscall__called_with_given_args_when_no_interrupted_task__expect_same_result_as_handler_is_returned(Aligned64Args, error);
    }

    #[test]
    fn on_syscall__called_when_args_is_addressable_by_interrupted_task_and_handler_returns_ok__expect_ok_is_returned() {
        let ok = Ok(());
        on_syscall__called_when_args_is_addressable_by_interrupted_task__expect_same_result_as_handler_is_returned(Dummy, ok);
    }

    fn on_syscall__called_when_args_is_addressable_by_interrupted_task__expect_same_result_as_handler_is_returned<A: SyscallArgs>(
        mut args: A,
        result: SyscallResult) {

        let error_as_usize = match result {
            Ok(()) => 0,
            Err(error) => NonZero::<usize>::from(error).get()
        };

        type StubbedSyscallHandler<T> = StubSyscallHandler<StubIsrContext<StubTaskScheduler<StubInterruptedTask>>, T>;

        let mut isr_context = StubIsrContext::stub_for_task_scheduler(
            StubTaskScheduler::stub_with_interrupted_task(
                StubInterruptedTask::stub_is_addressable_for(&raw mut args, true)));

        let _syscall_handler = StubbedSyscallHandler::stub_for(&isr_context, &mut args, result);
        let result = unsafe {
            <StubbedSyscallHandler<A> as SyscallIsrTrampoline<_>>::on_syscall(&mut isr_context, &raw mut args as usize)
        }.as_usize_result();

        expect!(result.as_usize()).to_equal(error_as_usize);
    }

    #[test]
    fn on_syscall__called_when_args_is_addressable_by_interrupted_task_and_handler_returns_err__expect_same_err_is_returned() {
        let error = Err(smeg_kernel::errors::test_doubles::any_kernel_error());
        on_syscall__called_when_args_is_addressable_by_interrupted_task__expect_same_result_as_handler_is_returned(Dummy, error);
    }

    #[test]
    fn on_syscall__called_when_args_is_addressible_by_interrupted_task_but_unaligned_and_handler_returns_ok__expect_ok_is_returned_because_is_addressable_broke_its_contract() {
        let ok = Ok(());
        let mut args = [Aligned2Args; 3];
        on_syscall__called_when_args_is_addressible_by_interrupted_task_but_unaligned__expect_same_result_as_handler_is_returned(&mut args[1], ok);

        let mut args = [Aligned4Args; 3];
        on_syscall__called_when_args_is_addressible_by_interrupted_task_but_unaligned__expect_same_result_as_handler_is_returned(&mut args[1], ok);

        let mut args = [Aligned64Args; 3];
        on_syscall__called_when_args_is_addressible_by_interrupted_task_but_unaligned__expect_same_result_as_handler_is_returned(&mut args[1], ok);
    }

    fn on_syscall__called_when_args_is_addressible_by_interrupted_task_but_unaligned__expect_same_result_as_handler_is_returned<A: HasSyscallId>(
        args: &mut A,
        result: SyscallResult) {

        let error_as_usize = match result {
            Ok(()) => 0,
            Err(error) => NonZero::<usize>::from(error).get()
        };

        type StubbedSyscallHandler<T> = StubSyscallHandler<StubIsrContext<StubTaskScheduler<StubInterruptedTask>>, T>;

        on_syscall__called_when_interrupted_task_and_unaligned_args__expect::<A, _>(|offset| {
            let unaligned_args_ptr = ((&raw mut *args as isize) + offset) as *mut A;
            let mut isr_context = StubIsrContext::stub_for_task_scheduler(
                StubTaskScheduler::stub_with_interrupted_task(
                    StubInterruptedTask::stub_is_addressable_for(unaligned_args_ptr, true)));

            let _syscall_handler = StubbedSyscallHandler::stub_for_ptr(&isr_context, unaligned_args_ptr, result);
            let result = unsafe {
                <StubbedSyscallHandler<A> as SyscallIsrTrampoline<_>>::on_syscall(&mut isr_context, unaligned_args_ptr as usize)
            }.as_usize_result();

            expect!(result.as_usize()).to_equal(error_as_usize);
        });
    }

    fn on_syscall__called_when_interrupted_task_and_unaligned_args__expect<A, F: FnMut(isize)>(mut assertion: F) {
        let bad_alignment = align_of::<A>() as isize - 1;
        for offset in -bad_alignment..=bad_alignment {
            if offset != 0 {
                assertion(offset);
            }
        }
    }
}
