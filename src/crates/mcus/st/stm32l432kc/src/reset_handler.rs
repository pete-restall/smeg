include!("../../../../os/mcu_bootstrapping.rs.inc");

unsafe extern "C" {
    static __LINKER_DATA_FLAGS_GUARANTEED_ZERO_ON_RESET_0: usize;
    static __LINKER_INITIAL_SP: usize;
}

core::arch::global_asm!(r#"
    .section .smeg.bootstrap.reset_handler, "ax"
    .global _reset_handler
    .type _reset_handler, %function
    .thumb_func

_reset_handler:
    eors r0, r0, r0
    ldr r1, ={0}
    str r0, [r1]
    ldr r0, {1}
    msr msp, r0
    b {2}
"#,
    sym __LINKER_DATA_FLAGS_GUARANTEED_ZERO_ON_RESET_0,
    sym __LINKER_INITIAL_SP,
    sym __smeg_os_entrypoint);
