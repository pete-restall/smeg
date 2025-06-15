use core::borrow::BorrowMut;

use smeg_kernel::bootstrapping::kernel::IsrBootstrapping;

use smeg_mcu_arm_cortex_m4_family::interrupts::{
    IsrContextImpl as CortexM4IsrContext,
    IsrVectorTableBuilder as CortexM4IsrVectorTableBuilder
};

use smeg_mcu_st_stm32l432kc::interrupts::IsrVectorTableBuilder as Stm32IsrVectorTableBuilder;

pub struct Drivers;

// TODO: this file looks like it could be completely generic, given a list of drivers (that we can iterate over to register 'stuff' like ISRs)
// and the correct imports (so long as all MCUs, etc. follow a naming convention)

impl Drivers {
    pub(crate) const fn collect_isr_vectors<I>(isrs: Stm32IsrVectorTableBuilder<I>) -> Stm32IsrVectorTableBuilder<I>
        where
            I: IsrBootstrapping,
            I::IsrContext: From<CortexM4IsrContext> + BorrowMut<CortexM4IsrContext> {

        // need to iterate over each driver and call an associated const fn with the same signature as this one
        Stm32IsrVectorTableBuilder::<I> {
            cortex_m4: Self::collect_cortex_m4_isr_vectors(isrs.cortex_m4),
            ..isrs
        }
    }

    const fn collect_cortex_m4_isr_vectors<I>(isrs: CortexM4IsrVectorTableBuilder<I>) -> CortexM4IsrVectorTableBuilder<I>
        where
            I: IsrBootstrapping,
            I::IsrContext: From<CortexM4IsrContext> + BorrowMut<CortexM4IsrContext> {

        smeg_drivers_kernel_syscall::collect_isr_vectors(isrs)
    }
}
