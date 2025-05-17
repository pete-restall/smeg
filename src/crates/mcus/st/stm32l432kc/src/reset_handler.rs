include!("../../../../os/mcu_bootstrapping.rs.inc");

use core::mem::MaybeUninit;

use crate::blinky_blinky;

unsafe extern "C" {
    unsafe static __LINKER_INITIAL_SP: usize;

    // TODO: these are all temporary, as an example...
    unsafe static mut __LINKER_BSS_START: MaybeUninit<usize>;
    unsafe static mut __LINKER_BSS_PAST_END: MaybeUninit<usize>;
    unsafe static mut __LINKER_DATA_START: MaybeUninit<usize>;
    unsafe static mut __LINKER_DATA_PAST_END: MaybeUninit<usize>;
    unsafe static __LINKER_DATA_LMA_START: usize;
}

core::arch::global_asm!(r#"
    .section .smeg.bootstrap.reset_handler, "ax"
    .global _reset_handler
    .type _reset_handler, %function
    .thumb_func

_reset_handler:
    ldr r0, {0}
    msr msp, r0
    b {1}
    b _reset_handler
"#,
    sym __LINKER_INITIAL_SP,
    sym __smeg_os_entrypoint);

// TODO: Eventually when proper symbols are used, this ought to be able to be deleted...
pub fn _needed_to_prevent_linker_gc() {
    smeg_mcu_arm_cortex_m4_family::needed_to_prevent_linker_gc();
}

#[inline(always)]
#[allow(static_mut_refs)]
unsafe extern "C" fn _reset_handler_impl() -> ! {
    // TODO: this is no longer used - it is left so that the contents can be copied / adapted into the new __smeg_os_entrypoint() / rust::RuntimeBootstrapping
    // strategy.
    //
    // The RuntimeBootstrapping implementation ought to use the __LINKER_* stuff and replace the cruft below:
    unsafe {
///////////// TODO: WRAP THESE INITIALISATIONS AND ZERO / COPY AS WE ARE DOING; NOTE THAT WE CAN 'PANIC' (SOMEHOW) IF THE POINTERS ARE WRONG, BUT WE
///////////// MUST NOT RELY ON ANY STATICS / ASSUMPTIONS OF ANYTHING BEING INITIALISED, SINCE CLEARLY IT IS NOT... THERE ARE ONLY TWO WAYS TO HANDLE THESE
///////////// PRE-RUNTIME INITIALISATION PANICS.  THE FIRST IS BY LOOPING FOREVER, SINCE THEY ARE COMPLETELY IRRECOVERABLE AND A RESET CAN NEVER FIX THIS,
///////////// AND THE SECOND IS TO EXECUTE SOME APPLICATION-SPECIFIC (PROBABLY ASSEMBLY) STUB THAT MAYBE TOGGLES A GPIO AS A PERPETUAL 'SOS' OR SOMETHING.

        let bss_start = __LINKER_BSS_START.as_mut_ptr();
        let bss_end =  __LINKER_BSS_PAST_END.as_mut_ptr();
        let bss_size_words = (bss_end as isize - bss_start as isize) / (usize::BITS as isize / 8);
        // assert / panic if bss_size_words < 0 !
        bss_start.write_bytes(0u8, bss_size_words as usize);

        // TODO: similar thing for the .data section
        let data_lma_start =  core::ptr::from_ref(&__LINKER_DATA_LMA_START);
        let data_start = __LINKER_DATA_START.as_mut_ptr();
        let data_end = __LINKER_DATA_PAST_END.as_mut_ptr();
        let data_size_words = (data_end as isize - data_start as isize) / (usize::BITS as isize / 8);
        // assert / panic if data_size_words < 0 !
        core::ptr::copy_nonoverlapping(data_lma_start, data_start, data_size_words as usize);

        // TODO: fire off each of the .init functions

        // TODO: call into the OS's (extern) entrypoint, with a bunch of MCU-specific stuff injected; this entrypoint will also be ! return, so
        // hopefully an optimised tailcall.  Currently just a blinky-blinky to make sure the code links and runs properly on the Nucleo board.
        blinky_blinky::blinky_blinky();
    }
}
