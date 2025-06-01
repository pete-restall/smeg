use std::any::Any;
use std::borrow::Cow;
use std::cell::Cell;
use std::thread;

use fluent_test::backend::AssertionSentence;
use fluent_test::prelude::*;

use smeg_kernel::errors::{KernelError, KernelErrorCode};

pub trait DespairMatchers<T: Fn() + Clone + Send> {
    fn to_despair(self) -> Self;
    fn to_despair_with_error_code(self, error_code: KernelErrorCode) -> Self;
}

thread_local! {
    static EXPECTED_ERROR_CODE: Cell<Option<KernelErrorCode>> = panic!("Thread-local EXPECTED_ERROR_CODE has not been initialised");
}

impl<T: Fn() + Clone + Send> DespairMatchers<T> for Assertion<T> {
    fn to_despair(self) -> Self {
        let was_successful = despairs_with_error_code(&self, None);
        let sentence = AssertionSentence::new("despair", "");
        self.add_step(sentence, was_successful)
    }

    fn to_despair_with_error_code(self, error_code: KernelErrorCode) -> Self {
        let was_successful = despairs_with_error_code(&self, Some(error_code));
        let sentence = AssertionSentence::new("despair", format!("with error code {error_code:?}"));
        self.add_step(sentence, was_successful)
    }
}

fn despairs_with_error_code<T: Fn() + Clone + Send>(assertion: &Assertion<T>, error_code: Option<KernelErrorCode>) -> bool {
    let was_successful = thread::scope(|scope| {
        let is_expecting_despair = !assertion.negated;
        let action = assertion.value.clone();
        let result = scope.spawn(move || {
            EXPECTED_ERROR_CODE.set(error_code);
            action();
        }).join();

        was_expectation_met_for(result, is_expecting_despair)
    });

    was_successful
}

fn was_expectation_met_for(result: Result<(), Box<dyn Any + Send>>, is_expecting_despair: bool) -> bool {
    let thread_panic_reason = &*panic_reason_from(result);
    (is_expecting_despair && thread_panic_reason == "Thread despaired") ||
    (!is_expecting_despair && thread_panic_reason == "Thread did not despair")
}

fn panic_reason_from<'a>(result: Result<(), Box<dyn Any + Send + 'a>>) -> Cow<'a, str> {
    use smeg_testing_host_utils::threads::PanicReason;
    result.panic_reason().unwrap_or(Cow::from("Thread did not despair"))
}

#[unsafe(no_mangle)]
pub unsafe fn __smeg_is_in_despair(squid: KernelError) -> ! {
    if let Some(error_code) = EXPECTED_ERROR_CODE.get() {
        expect!(squid.code).to_equal(error_code);
    }

    panic!("Thread despaired");
}
