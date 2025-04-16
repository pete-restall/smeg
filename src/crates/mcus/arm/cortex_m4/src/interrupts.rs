type IsrVectorTableEntry = unsafe extern "C" fn() -> !;

unsafe extern "C" {
    unsafe fn _reset_handler() -> !;
}

unsafe extern "C" fn unhandled_isr() -> ! {
    loop { }
}

#[used]
#[unsafe(link_section = ".smeg.isr_vector_table.arm.cortex_m4")]
static ISR_VECTOR_TABLE: [IsrVectorTableEntry; 15] = [
    _reset_handler,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr,
    unhandled_isr
];
