use smeg_config::SMEG_CONFIG;

const STACK_SIZE_WORDS: usize = SMEG_CONFIG.VALUES.KERNEL.STACK.SIZE_IN_WORDS as usize;
const _: () = assert!(
    STACK_SIZE_WORDS >= 64,
    "Kernel stack size is unrealistically small; even a single context saving just core registers requires 28 words.  ISR stack \
    usage also needs to be taken into account.");

const _: () = assert!(STACK_SIZE_WORDS & 7 == 0, "Kernel stack size needs to be a multiple of 2 words (8 bytes).");

#[cfg(feature = "power_standby")]
const _: () = assert!(
    STACK_SIZE_WORDS <= 4096,
    "Kernel stack size is too large for SRAM2; try disabling support for standby power-saving mode if you really need this much.  \
    Note that for standby mode there are other data structures that need to fit into SRAM2, so a full 4096 word reservation for \
    the kernel stack is still unrealistic.  This assertion is a sense-check only, but expect linker errors if operating close to \
    the limit.");

#[cfg(not(feature = "power_standby"))]
const _: () = assert!(
    STACK_SIZE_WORDS <= 8192,
    "Kernel stack size is too large for SRAM.  Note that a full 8192 word reservation is unrealistic as other data structures \
    will also require storage.  This assertion is a sense-check only, but expect linker errors if operating close to the limit.");

#[used]
#[unsafe(link_section = ".kernel.stack.0")]
pub static RAW: [usize; STACK_SIZE_WORDS] = [0; STACK_SIZE_WORDS];
