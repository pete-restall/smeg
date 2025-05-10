#![cfg_attr(not(any(test, feature = "std")), no_std)]

use std::{io, thread};
use std::thread::JoinHandle;

use smeg_config::SMEG_CONFIG;

const NUMBER_OF_CORES: usize = SMEG_CONFIG.VALUES.MCUS.HOST.RUST_STD.NUMBER_OF_CORES as usize;
const _: () = assert!(NUMBER_OF_CORES >= 1, "Number of simulated MCU cores must be at least 1.");
const _: () = assert!(
    NUMBER_OF_CORES <= 16,
    "Number of simulated MCU cores probably should be less than 16; this is an artificial limit for sense-checking purposes only, so feel free \
    to tweak the asserted limit if necessary.");

const STACK_SIZE_WORDS: usize = SMEG_CONFIG.VALUES.KERNEL.STACK.SIZE_IN_WORDS as usize;
const _: () = assert!(STACK_SIZE_WORDS >= 4096, "Kernel stack size is unrealistically small.");

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    smeg_kernel::panic_handler::claim_std_panic_hook();

    let mut core_threads = (0..NUMBER_OF_CORES)
        .map(spawn_thread_for_core)
        .collect::<io::Result<Vec<_>>>()
        .unwrap();

    let _all_joined = core_threads
        .drain(..)
        .map(move |thread| thread.join())
        .collect::<thread::Result<Vec<_>>>()
        .unwrap();

    0
}

fn spawn_thread_for_core(core_id: usize) -> std::io::Result<JoinHandle<()>> {
    spawn_thread_for_core_and_entrypoint(
        core_id,
        if core_id == 0 {
            primary_core_entrypoint
        } else {
            secondary_core_entrypoint
        })
}

fn spawn_thread_for_core_and_entrypoint<F>(core_id: usize, entrypoint: F) -> std::io::Result<JoinHandle<()>>
    where F: Send + FnOnce(usize) -> () + 'static {

    thread::Builder::new()
        .stack_size(STACK_SIZE_WORDS * size_of::<usize>())
        .name(format!("mcu-core-{core_id}"))
        .spawn(move || entrypoint(core_id))
}

fn primary_core_entrypoint(_core_id: usize) {
    // TODO: This will call into a well-known exposed smeg-os hook with a given bootstrapping trait describing the primary core.  The
    // entrypoint for the primary core needs to initialise BSS, data, and any other global / static 'stuff' before setting a flag to
    // wake the secondary cores from their sleep and allow them to initialise themselves.
    loop { /* The smeg-os hook will take some trait and not return (!), probably also #[inline] to reduce stack frame for smaller devices */ }
}

fn secondary_core_entrypoint(_core_id: usize) {
    // TODO: This needs to do two things, yet to be decided.  The transfer to the OS entrypoint for secondary cores needs to be blocked
    // until after the primary entrypoint has signalled the environment (BSS, etc.) has been sufficiently initialised.  So:
    // 1. sleep until a flag is set that 'releases' the MCU, basically indicating that the primary core has been sufficiently initialised
    // 2. call into a well-known exposed smeg-os hook with a given bootstrapping trait describing the secondary core; this entrypoint does
    //    not have to initialise BSS, data, etc.
    loop { /* The smeg-os hook will take some trait and not return (!), probably also #[inline] to reduce stack frame for smaller devices */ }
}
