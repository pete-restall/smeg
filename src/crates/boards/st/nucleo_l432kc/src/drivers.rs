use smeg_mcu_arm_cortex_m4_family::interrupts::IsrVectorTableBuilder as CortexM4IsrVectorTableBuilder;

use smeg_mcu_st_stm32l432kc::interrupts::IsrVectorTableBuilder as Stm32IsrVectorTableBuilder;

type SyscallDriver = smeg_drivers_kernel_syscall::Driver<DriverDependencies>;

pub struct Drivers;

struct DriverDependencies;

trait HasIsrContext {
    type IsrContext: smeg_kernel::interrupts::IsrContext;
}

impl HasIsrContext for DriverDependencies {
    type IsrContext = smeg_mcu_st_stm32l432kc::interrupts::IsrContextImpl;
}

impl smeg_drivers_kernel_syscall::Dependencies for DriverDependencies {
    type IsrContext = <Self as HasIsrContext>::IsrContext;
}

// TODO: temporary way to thrash out how a Driver should look whilst also verifying the code by blinking the LED on the Nucleo board
cfg_if::cfg_if! {
    if #[cfg(target_arch = "arm")] {
        use smeg_drivers_kernel_syscall::syscall_map;
        type BlinkyBlinkyDriver = super::blinky_blinky::Driver<DriverDependencies>;
        impl super::blinky_blinky::Dependencies for DriverDependencies { type IsrContext = <Self as HasIsrContext>::IsrContext; }
        syscall_map! { BlinkyBlinkySyscall -> <BlinkyBlinkyDriver as super::blinky_blinky::Syscalls>::BlinkyBlinkySyscallHandler }
    }
}

// TODO: does each driver need to export a macro that takes a bunch of dependencies, and then instantiates it ?  Knowing the concrete types would allow
// the macro to create 'generic' statics (eg. the SyscallHandler trampoline tables)

// TODO: this file looks like it could be completely generic, given a list of drivers (that we can iterate over to register 'stuff' like ISRs)
// and the correct imports (so long as all MCUs, etc. follow a naming convention)

impl Drivers {
    pub(crate) const fn collect_isr_vectors(isrs: Stm32IsrVectorTableBuilder) -> Stm32IsrVectorTableBuilder {
        Stm32IsrVectorTableBuilder {
            cortex_m4: Self::collect_cortex_m4_isr_vectors(isrs.cortex_m4),
            ..isrs
        }
    }

    const fn collect_cortex_m4_isr_vectors(isrs: CortexM4IsrVectorTableBuilder) -> CortexM4IsrVectorTableBuilder {
        SyscallDriver::collect_isr_vectors(isrs)
    }
}
