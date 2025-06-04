use smeg_mcu_st_stm32l432kc::define_isr_vector_table_from;
use smeg_mcu_st_stm32l432kc::interrupts::IsrVectorTable;

struct Drivers;

// TODO: this file looks like it could be completely generic, given a list of drivers (that we can iterate over to register 'stuff' like ISRs)
// and the correct imports (so long as all MCUs, etc. follow a naming convention)

impl Drivers {
    const fn collect_isr_vectors<T>(isrs: T) -> T {
        isrs // need to iterate over each driver and call an associated const fn with the same signature as this one
    }
}

define_isr_vector_table_from!(Drivers::collect_isr_vectors(IsrVectorTable::default()));
