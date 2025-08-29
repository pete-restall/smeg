use std::cell::Cell;
use std::{io, thread, time};
use std::num::NonZero;

use smeg_config::SMEG_CONFIG;
use smeg_kernel::{ConstUsize, HasConstUsizeValue, HasMcuCoreId};

#[derive(Copy, Clone)]
pub struct McuCore {
    id: usize,
    kernel_stack_size_words: NonZero<usize>
}

impl McuCore {
    thread_local! {
        static TLS: Cell<McuCore> = panic!("McuCore TLS has not been initialised");
    }

    pub(crate) const NUMBER_OF_MCU_CORES: NonZero<usize> = NonZero::new(<<Self as HasMcuCoreId>::NumberOfMcuCores>::VALUE).unwrap();
    const MIN_KERNEL_STACK_SIZE_WORDS: NonZero<usize> = NonZero::new(4096).unwrap();

    pub fn try_new(id: usize, kernel_stack_size_words: NonZero<usize>) -> Result<McuCore, String> {
        if id >= Self::NUMBER_OF_MCU_CORES.get() {
            Err(format!("Core ID is out of range; id={}, NUMBER_OF_MCU_CORES={}", id, Self::NUMBER_OF_MCU_CORES.get()))
        } else if kernel_stack_size_words < Self::MIN_KERNEL_STACK_SIZE_WORDS {
            Err(format!(
                "Kernel stack size is unrealistically small; kernel_stack_size_words={}, MIN_KERNEL_STACK_SIZE_WORDS={}",
                kernel_stack_size_words,
                Self::MIN_KERNEL_STACK_SIZE_WORDS))
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
            .stack_size(self.kernel_stack_size_words.get() * size_of::<usize>())
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

const NUMBER_OF_MCU_CORES: usize = {
    const VALUE: i64 = SMEG_CONFIG.VALUES.MCUS.HOST.RUST_STD.NUMBER_OF_CORES;
    assert!(VALUE >= 1, "Number of simulated MCU cores must be at least 1.");
    assert!(VALUE <= usize::BITS as i64, "Number of simulated MCU cores must be no more than the number of bits in a usize (optimisation).");
    VALUE as usize
};

impl HasMcuCoreId for McuCore {
    type NumberOfMcuCores = ConstUsize<NUMBER_OF_MCU_CORES>;

    fn mcu_core_id(&self) -> usize { self.id }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub(crate) mod tests {
    use core::sync::atomic::{AtomicBool, Ordering};

    use fluent_test::prelude::*;

    use smeg_testing_host_utils::integers::any_usize_within;

    use super::*;

    #[test]
    fn try_new__called_with_mcu_core_id_greater_than_or_equal_to_number_of_cores__expect_err() {
        let number_of_mcu_cores = McuCore::NUMBER_OF_MCU_CORES.get();
        [number_of_mcu_cores, number_of_mcu_cores + 1, number_of_mcu_cores + 10].iter().for_each(|bad_core_id| {
            let err = McuCore::try_new(*bad_core_id, McuCore::MIN_KERNEL_STACK_SIZE_WORDS).err().expect("must be Err<String>");
            expect!(err).to_contain("ore ID");
        });
    }

    #[test]
    fn try_new__called_with_stack_size_words_less_than_minimum__expect_err() {
        [1, McuCore::MIN_KERNEL_STACK_SIZE_WORDS.get() - 2, McuCore::MIN_KERNEL_STACK_SIZE_WORDS.get() - 1].iter().for_each(|too_few_words| {
            let too_few_words = NonZero::new(*too_few_words).unwrap();
            let err = McuCore::try_new(any_mcu_core_id(), too_few_words).err().expect("must be Err<String>");
            expect!(err).to_contain("stack size");
        });
    }

    fn any_mcu_core_id() -> usize {
        any_usize_within(0..McuCore::NUMBER_OF_MCU_CORES.get())
    }

    #[test]
    fn try_new__called_with_stack_size_words_equal_to_minimum__expect_ok() {
        let result = McuCore::try_new(any_mcu_core_id(), McuCore::MIN_KERNEL_STACK_SIZE_WORDS);
        expect!(result.is_ok()).to_be_true();
    }

    #[test]
    fn NUMBER_OF_MCU_CORES__get__expect_value_from_config() {
        expect!(McuCore::NUMBER_OF_MCU_CORES.get()).to_equal(SMEG_CONFIG.VALUES.MCUS.HOST.RUST_STD.NUMBER_OF_CORES as usize);
    }

    #[test]
    fn mcu_core_id__get__expect_same_value_passed_to_constructor() {
        let id = any_mcu_core_id();
        let mcu_core = McuCore::try_new(id, any_kernel_stack_size_words()).expect("must be Ok<McuCore>");
        expect!(mcu_core.id).to_equal(id);
    }

    fn any_kernel_stack_size_words() -> NonZero<usize> {
        let min_words = McuCore::MIN_KERNEL_STACK_SIZE_WORDS.get();
        NonZero::new(any_usize_within(min_words..min_words + 100)).unwrap()
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

    fn stub_tls_kernel_stack_size_words(kernel_stack_size_words: NonZero<usize>) {
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
        any_usize_within(1..McuCore::NUMBER_OF_MCU_CORES.get())
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
