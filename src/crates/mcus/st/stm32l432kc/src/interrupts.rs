type IsrVectorTableEntry = unsafe extern "C" fn() -> !;

unsafe extern "C" fn unhandled_isr() -> ! {
    loop { }
}

#[used]
#[unsafe(link_section = ".smeg.isr_vector_table.st.stm32l432kc")]
static ISR_VECTOR_TABLE: [IsrVectorTableEntry; 85] = [unhandled_isr; 85];
