include!("../../../../os/mcu_bootstrapping.rs.inc");

unsafe extern "C" {
    static __LINKER_INITIAL_SP: usize;
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
"#,
    sym __LINKER_INITIAL_SP,
    sym __smeg_os_entrypoint);
