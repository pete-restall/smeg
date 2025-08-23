include!("../../../../../os/mcu_bootstrapping.rs.inc");

use std::{io, thread};
use std::num::NonZero;

use smeg_config::SMEG_CONFIG;
use smeg_kernel::HasMcuCoreId;

use crate::McuCore;

const KERNEL_STACK_SIZE_WORDS: usize = SMEG_CONFIG.VALUES.KERNEL.STACK.SIZE_IN_WORDS as usize;
const _: () = assert!(KERNEL_STACK_SIZE_WORDS >= 4096, "Kernel stack size is unrealistically small.");

pub fn entrypoint() -> Result<isize, String> {
    smeg_kernel::panic_handler::claim_std_panic_hook();

    let mut mcu_cores = (0..McuCore::NUMBER_OF_MCU_CORES.get())
        .map(|core_id| McuCore::try_new(core_id, NonZero::new(KERNEL_STACK_SIZE_WORDS).unwrap()))
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
