use crate::blinky_blinky;

unsafe extern "C" {
    unsafe static __LINKER_INITIAL_SP: usize;

    // TODO: these are all temporary, as an example...
    unsafe static mut __LINKER_BSS_START: usize;
    unsafe static mut __LINKER_BSS_PAST_END: usize;
    unsafe static mut __LINKER_DATA_START: usize;
    unsafe static mut __LINKER_DATA_PAST_END: usize;
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
    sym _reset_handler_impl);

#[inline(always)]
#[allow(static_mut_refs)]
unsafe extern "C" fn _reset_handler_impl() -> ! {
    // TODO: since this function will largely be generic across MCUs, this stuff should be passed to a 'smeg runtime init' or somesuch in
    // the kernel, parameterised for the various things that need passing in.
    unsafe {
        let bss_start = core::ptr::from_mut(&mut __LINKER_BSS_START);
        let bss_end = core::ptr::from_mut(&mut __LINKER_BSS_PAST_END);
        let bss_size = bss_end.offset_from(bss_start);
        // assert / panic if start > end, sbss_size < 0, etc. ?
        bss_start.write_bytes(0u8, bss_size as usize);

        // TODO: similar thing for the .data section
        let data_lma_start = core::ptr::from_ref(&__LINKER_DATA_LMA_START);
        let data_start = core::ptr::from_mut(&mut __LINKER_DATA_START);
        let data_end = core::ptr::from_mut(&mut __LINKER_DATA_PAST_END);
        let data_size = data_end.offset_from(data_start);
        core::ptr::copy_nonoverlapping(data_lma_start, data_start, data_size as usize);

        // TODO: fire off each of the .init functions

        // TODO: call into the OS's (extern) entrypoint, with a bunch of MCU-specific stuff injected; this entrypoint will also be ! return, so
        // hopefully an optimised tailcall.  Currently just a blinky-blinky to make sure the code links and runs properly on the Nucleo board.
        blinky_blinky::blinky_blinky();
    }
}
