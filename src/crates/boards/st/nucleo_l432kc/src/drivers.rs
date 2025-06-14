use smeg_kernel::bootstrapping::kernel::IsrBootstrapping;

use smeg_mcu_arm_cortex_m4_family::interrupts::IsrVectorTableBuilder as CortexM4IsrVectorTableBuilder;

use smeg_mcu_st_stm32l432kc::interrupts::IsrVectorTableBuilder as Stm32IsrVectorTableBuilder;

pub struct Drivers;

// TODO: this file looks like it could be completely generic, given a list of drivers (that we can iterate over to register 'stuff' like ISRs)
// and the correct imports (so long as all MCUs, etc. follow a naming convention)

impl Drivers {
    pub(crate) const fn collect_isr_vectors<I: IsrBootstrapping>(isrs: Stm32IsrVectorTableBuilder<I>) -> Stm32IsrVectorTableBuilder<I> {
        // need to iterate over each driver and call an associated const fn with the same signature as this one
        Stm32IsrVectorTableBuilder::<I> {
            cortex_m4: Self::collect_cortex_m4_isr_vectors(isrs.cortex_m4),
            ..isrs
        }
    }

    const fn collect_cortex_m4_isr_vectors<I: IsrBootstrapping>(isrs: CortexM4IsrVectorTableBuilder<I>) -> CortexM4IsrVectorTableBuilder<I> {
        smeg_drivers_kernel_syscall::Driver::collect_isr_vectors(isrs)
    }
}
