include!("../../../../../os/mcu_bootstrapping.rs.inc");

use std::{io, thread};

use smeg_config::SMEG_CONFIG;

use crate::McuCore;

const NUMBER_OF_CORES: usize = SMEG_CONFIG.VALUES.MCUS.HOST.RUST_STD.NUMBER_OF_CORES as usize;
const _: () = assert!(NUMBER_OF_CORES >= 1, "Number of simulated MCU cores must be at least 1.");
const _: () = assert!(
    NUMBER_OF_CORES <= 16,
    "Number of simulated MCU cores probably should be less than 16; this is an artificial limit for sense-checking purposes only, so feel free \
    to tweak the asserted limit if necessary.");

const KERNEL_STACK_SIZE_WORDS: usize = SMEG_CONFIG.VALUES.KERNEL.STACK.SIZE_IN_WORDS as usize;
const _: () = assert!(KERNEL_STACK_SIZE_WORDS >= 4096, "Kernel stack size is unrealistically small.");

pub fn entrypoint() -> Result<isize, String> {
    smeg_kernel::panic_handler::claim_std_panic_hook();

    let mut mcu_cores = (0..NUMBER_OF_CORES)
        .map(|core_id| McuCore::try_new(core_id, KERNEL_STACK_SIZE_WORDS))
        .collect::<Result<Vec<_>, String>>()?;

    thread::scope(|scope| -> Result<isize, String> {
        let mut mcu_core_threads = mcu_cores
            .drain(..)
            .map(|mcu_core| mcu_core.as_thread(scope, || unsafe { __smeg_os_entrypoint() }))
            .collect::<io::Result<Vec<_>>>()
            .map_err(|err| err.to_string())?;

        let _all_joined = mcu_core_threads
            .drain(..)
            .map(move |thread| thread.join())
            .collect::<thread::Result<Vec<_>>>()
            .map_err(thread_err_to_string)?;

        Ok(0)
    })
}

fn thread_err_to_string(err: Box<dyn std::any::Any + Send + 'static>) -> String {
    match (err.downcast_ref::<&str>(), err.downcast_ref::<String>()) {
        (Some(s), _) => s.to_string(),
        (_, Some(s)) => s.to_string(),
        _ => "Unknown error from joined thread".to_string()
    }
}
