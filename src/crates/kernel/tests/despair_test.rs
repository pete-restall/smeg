#![allow(non_snake_case)]

use std::any::Any;
use std::borrow::Cow;
use std::cell::Cell;
use std::thread;

use fluent_test::prelude::*;

use smeg_kernel::despair;
use smeg_kernel::errors::{KernelError, KernelErrorCode};

use smeg_testing_host_utils::seq::any_item_from;

thread_local! {
    static EXPECTED_ERROR_CODE: Cell<KernelErrorCode> = panic!("Thread-local EXPECTED_ERROR_CODE has not been initialised");
}

#[test]
fn despair__called_using_with_and_because__expect_despair_handler_is_called_with_same_error_code() {
    despair__called__expect_despair_handler_is_called_with_same_error_code(|| {
        let error_code = any_error_code();
        EXPECTED_ERROR_CODE.set(error_code);
        despair!(with(error_code), because("something", "bad", "happened"));
    });
}

fn despair__called__expect_despair_handler_is_called_with_same_error_code(despair: fn()) {
    let result = thread::spawn(move || {
        despair();
        #[allow(unreachable_code)] { unreachable!("because despair!(...) should never return") }
    }).join();

    let thread_panic_reason = &*panic_reason_from(result);
    expect!(thread_panic_reason).to_equal("Despairing thread assertion OK");
}

fn panic_reason_from<'a>(result: Result<(), Box<dyn Any + Send + 'a>>) -> Cow<'a, str> {
    match result {
        Ok(()) => Cow::from("Despairing thread exited normally - this shouldn't happen"),
        Err(err) => match (err.downcast_ref::<&'a str>(), err.downcast_ref::<&'a String>()) {
            (Some(s), _) => Cow::from(*s),
            (_, Some(s)) => Cow::from(*s),
            _ => Cow::from("Unknown error from joined thread")
        }
    }
}

fn any_error_code() -> KernelErrorCode {
    *any_item_from(&[
        KernelErrorCode::GeneralDespair,
        KernelErrorCode::LinkerScriptDespair])
}

#[unsafe(no_mangle)]
unsafe extern "Rust" fn __smeg_is_in_despair(squid: KernelError) -> ! {
    expect!(squid.code).to_equal(EXPECTED_ERROR_CODE.get());
    panic!("Despairing thread assertion OK");
}

#[test]
fn despair__called_using_with__expect_despair_handler_is_called_with_same_error_code() {
    despair__called__expect_despair_handler_is_called_with_same_error_code(|| {
        let error_code = any_error_code();
        EXPECTED_ERROR_CODE.set(error_code);
        despair!(with(error_code));
    });
}

#[test]
fn despair__called_using_because__expect_despair_handler_is_called_with_error_code_for_general_despair() {
    despair__called__expect_despair_handler_is_called_with_same_error_code(|| {
        EXPECTED_ERROR_CODE.set(KernelErrorCode::GeneralDespair);
        despair!(because("general despair is to be expected"));
    });
}
