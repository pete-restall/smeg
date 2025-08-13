use std::cell::Cell;
use std::{io, thread, time};

use smeg_kernel::HasMcuCoreId;

#[derive(Copy, Clone)]
pub struct McuCore {
    id: usize,
    kernel_stack_size_words: usize
}

impl McuCore {
    thread_local! {
        static TLS: Cell<McuCore> = panic!("McuCore TLS has not been initialised");
    }

    const MIN_KERNEL_STACK_SIZE_WORDS: usize = 4096;

    pub fn try_new(id: usize, kernel_stack_size_words: usize) -> Result<McuCore, String> {
        if kernel_stack_size_words < Self::MIN_KERNEL_STACK_SIZE_WORDS {
            Err("Kernel stack size is unrealistically small.".to_string())
        } else {
            Ok(McuCore {
                id,
                kernel_stack_size_words
            })
        }
    }

    pub fn as_thread<'scope, 'env, F>(
        self,
        scope: &'scope thread::Scope<'scope, 'env>,
        entrypoint: F) -> io::Result<thread::ScopedJoinHandle<'scope, ()>>
        where F: FnOnce() + Send + 'scope {

        thread::Builder::new()
            .stack_size(self.kernel_stack_size_words * size_of::<usize>())
            .name(format!("mcu-core-{}", self.id))
            .spawn_scoped(scope, move || {
                Self::TLS.set(self);
                if self.id == 0 {
                    Self::primary_core(entrypoint);
                } else {
                    Self::secondary_core(entrypoint);
                }
            })
    }

    fn primary_core<F: FnOnce()>(entrypoint: F) {
        entrypoint()
    }

    fn secondary_core<F: FnOnce()>(entrypoint: F) {
        // TODO: we need to block here, waiting for some sort of 'wake up' signal from the OS to say that the primary core has been
        // initialised to a suitable extent (ie. the Rust runtime has been initialised and any other OS state that ensures it is safe
        // to bring further MCU cores online).  For now we'll just sleep for a second...
        thread::sleep(time::Duration::from_secs(1));
        entrypoint()
    }
}

impl Default for McuCore {
    fn default() -> Self {
        let tls = McuCore::TLS.get();
        Self {
            id: tls.id,
            kernel_stack_size_words: tls.kernel_stack_size_words
        }
    }
}

impl HasMcuCoreId for McuCore {
    fn mcu_core_id(&self) -> usize { self.id }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub(crate) mod tests {
    use core::sync::atomic::{AtomicBool, Ordering};

    use fluent_test::prelude::*;

    use smeg_testing_host_utils::integers::{any_usize, any_usize_within};
    use super::*;

    #[test]
    fn try_new__called_with_stack_size_words_less_than_minimum__expect_err() {
        [0, McuCore::MIN_KERNEL_STACK_SIZE_WORDS - 2, McuCore::MIN_KERNEL_STACK_SIZE_WORDS - 1].iter().for_each(|too_few_words| {
            let err = McuCore::try_new(any_mcu_core_id(), *too_few_words).err().expect("must be Err<String>");
            expect!(err).to_contain("stack size");
        });
    }

    fn any_mcu_core_id() -> usize {
        any_usize()
    }

    #[test]
    fn try_new__called_with_stack_size_words_equal_to_minimum__expect_ok() {
        let result = McuCore::try_new(any_mcu_core_id(), McuCore::MIN_KERNEL_STACK_SIZE_WORDS);
        expect!(result.is_ok()).to_be_true();
    }

    #[test]
    fn mcu_core_id__get__expect_same_value_passed_to_constructor() {
        let id = any_mcu_core_id();
        let mcu_core = McuCore::try_new(id, any_kernel_stack_size_words()).expect("must be Ok<McuCore>");
        expect!(mcu_core.id).to_equal(id);
    }

    fn any_kernel_stack_size_words() -> usize {
        any_usize_within(McuCore::MIN_KERNEL_STACK_SIZE_WORDS..McuCore::MIN_KERNEL_STACK_SIZE_WORDS + 100)
    }

    #[test]
    fn mcu_core_id__get_from_default_instance__expect_same_value_as_tls_core_id() {
        let tls_id = any_mcu_core_id();
        stub_tls_mcu_core_id(tls_id);
        expect!(McuCore::default().mcu_core_id()).to_equal(tls_id);
    }

    fn stub_tls_mcu_core_id(id: usize) {
        let mcu_core = McuCore::try_new(id, any_kernel_stack_size_words()).expect("must be Ok<McuCore>");
        McuCore::TLS.set(mcu_core);
    }

    #[test]
    fn kernel_stack_size_words__get__expect_same_value_passed_to_constructor() {
        let kernel_stack_size_words = any_kernel_stack_size_words();
        let mcu_core = McuCore::try_new(any_mcu_core_id(), kernel_stack_size_words).expect("must be Ok<McuCore>");
        expect!(mcu_core.kernel_stack_size_words).to_equal(kernel_stack_size_words);
    }

    #[test]
    fn kernel_stack_size_words__get_from_default_instance__expect_same_value_as_tls_stack_size() {
        let tls_kernel_stack_size_words = any_kernel_stack_size_words();
        stub_tls_kernel_stack_size_words(tls_kernel_stack_size_words);
        expect!(McuCore::default().kernel_stack_size_words).to_equal(tls_kernel_stack_size_words);
    }

    fn stub_tls_kernel_stack_size_words(kernel_stack_size_words: usize) {
        let mcu_core = McuCore::try_new(any_mcu_core_id(), kernel_stack_size_words).expect("must be Ok<McuCore>");
        McuCore::TLS.set(mcu_core);
    }

    #[test]
    fn mcu_core_id__called__expect_same_value_passed_to_constructor() {
        let id = any_mcu_core_id();
        let mcu_core = McuCore::try_new(id, any_kernel_stack_size_words()).expect("must be Ok<McuCore>");
        expect!(mcu_core.mcu_core_id()).to_equal(id);
    }

    #[test]
    fn as_thread__called_when_core_id_is_zero__expect_entrypoint_is_called() {
        as_thread__called_with_core_id__expect_entrypoint_is_called(0);
    }

    fn as_thread__called_with_core_id__expect_entrypoint_is_called(core_id: usize) {
        let entrypoint_was_called = AtomicBool::new(false);
        thread::scope(|scope| {
            let mcu_core = McuCore::try_new(core_id, any_kernel_stack_size_words()).expect("must be Ok<McuCore>");
            let mcu_core_thread = mcu_core.as_thread(
                scope,
                || entrypoint_was_called.store(true, Ordering::Relaxed)).expect("must be Ok<ScopedJoinHandle>");

            mcu_core_thread.join().expect("must be Ok<()>");
        });

        expect!(entrypoint_was_called.load(Ordering::Relaxed)).to_be_true();
    }

    #[test]
    fn as_thread__called_when_core_id_is_nonzero__expect_entrypoint_is_called() {
        as_thread__called_with_core_id__expect_entrypoint_is_called(any_nonzero_mcu_core_id());
    }

    fn any_nonzero_mcu_core_id() -> usize {
        any_usize_within(1..=usize::MAX)
    }

    #[test]
    fn as_thread__called_when_core_id_is_zero__expect_core_id_is_set_in_tls_before_entrypoint_is_called() {
        let mcu_core = McuCore::try_new(0, any_kernel_stack_size_words()).expect("must be Ok<McuCore>");
        let spied = spy_tls_when_entrypoint_called_for(mcu_core);
        expect!(spied.id).to_equal(0);
    }

    fn spy_tls_when_entrypoint_called_for(mcu_core: McuCore) -> McuCore {
        let mut spied_mcu_core = Option::<McuCore>::None;
        thread::scope(|scope| {
            let mcu_core_thread = mcu_core.as_thread(
                scope,
                || spied_mcu_core = Some(McuCore::TLS.get())).expect("must be Ok<ScopedJoinHandle>");

            mcu_core_thread.join().expect("must be Ok<()>");
        });

        spied_mcu_core.expect("must be Some<McuCore>")
    }

    #[test]
    fn as_thread__called_when_core_id_is_nonzero__expect_core_id_is_set_in_tls_before_entrypoint_is_called() {
        let id = any_nonzero_mcu_core_id();
        let mcu_core = McuCore::try_new(id, any_kernel_stack_size_words()).expect("must be Ok<McuCore>");
        let spied = spy_tls_when_entrypoint_called_for(mcu_core);
        expect!(spied.id).to_equal(id);
    }

    #[test]
    fn as_thread__called_when_core_id_is_zero__expect_kernel_stack_size_words_is_set_in_tls_before_entrypoint_is_called() {
        let kernel_stack_size_words = any_kernel_stack_size_words();
        let mcu_core = McuCore::try_new(0, kernel_stack_size_words).expect("must be Ok<McuCore>");
        let spied = spy_tls_when_entrypoint_called_for(mcu_core);
        expect!(spied.kernel_stack_size_words).to_equal(kernel_stack_size_words);
    }

    #[test]
    fn as_thread__called_when_core_id_is_nonzero__expect_kernel_stack_size_words_is_set_in_tls_before_entrypoint_is_called() {
        let kernel_stack_size_words = any_kernel_stack_size_words();
        let mcu_core = McuCore::try_new(any_nonzero_mcu_core_id(), kernel_stack_size_words).expect("must be Ok<McuCore>");
        let spied = spy_tls_when_entrypoint_called_for(mcu_core);
        expect!(spied.kernel_stack_size_words).to_equal(kernel_stack_size_words);
    }

    #[test]
    fn as_thread__called_when_core_id_is_zero__expect_thread_is_named_nicely() {
        let mcu_core = McuCore::try_new(0, any_kernel_stack_size_words()).expect("must be Ok<McuCore>");
        let spied = spy_thread_name_when_entrypoint_called_for(mcu_core);
        expect!(spied).to_equal("mcu-core-0".to_string());
    }

    fn spy_thread_name_when_entrypoint_called_for(mcu_core: McuCore) -> String {
        let mut spied_thread_name = Option::<String>::None;
        thread::scope(|scope| {
            let mcu_core_thread = mcu_core.as_thread(
                scope,
                || if let Some(name) = thread::current().name() {
                    spied_thread_name = Some(name.to_owned())
                }).expect("must be Ok<ScopedJoinHandle>");

            mcu_core_thread.join().expect("must be Ok<()>");
        });

        spied_thread_name.expect("must be Some<String>")
    }

    #[test]
    fn as_thread__called_when_core_id_is_nonzero__expect_thread_is_named_nicely() {
        let id = any_nonzero_mcu_core_id();
        let mcu_core = McuCore::try_new(id, any_kernel_stack_size_words()).expect("must be Ok<McuCore>");
        let spied = spy_thread_name_when_entrypoint_called_for(mcu_core);
        expect!(spied).to_equal(format!("mcu-core-{id}"));
    }
}
