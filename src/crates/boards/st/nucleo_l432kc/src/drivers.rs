use core::convert::{AsMut, AsRef, From};

use smeg_kernel::tasks::HasTaskScheduler;
use smeg_mcu_arm_cortex_m4_family::interrupts::IsrVectorTableBuilder as CortexM4IsrVectorTableBuilder;
use smeg_mcu_st_stm32l432kc::interrupts::IsrVectorTableBuilder as Stm32IsrVectorTableBuilder;

mod mcu {
    use smeg_kernel::interrupts::HasIsrContext;

    pub type Driver = smeg_mcu_st_stm32l432kc::Driver;

    pub type IsrContext = <Driver as HasIsrContext>::IsrContext;
    // pub type FamilyIsrContext = <Driver as HasFamilyIsrContext>::FamilyIsrContext; // TODO: DOES NOT WORK DUE TO 'impl From<T> for T' IN core !
    pub type FamilyIsrContext = smeg_mcu_arm_cortex_m4_family::interrupts::IsrContext; // But this DOES work despite being the same type... :-/
}

mod syscall {
    use smeg_kernel::interrupts::HasIsrContext;

    pub struct Dependencies;
    impl smeg_drivers_kernel_syscall::Dependencies for Dependencies {
        type IsrContext = super::IsrContext;
    }

    pub type Driver = smeg_drivers_kernel_syscall::Driver<Dependencies>;

    pub type IsrContext = <Driver as HasIsrContext>::IsrContext;
}

mod task_scheduler {
    // TODO: Temporary until a proper task scheduler driver is introduced
    use smeg_kernel::IsAddressableMut;
    use smeg_kernel::tasks::{HasInterruptedTask, HasInterruptedTaskMut};

    pub struct Driver;

    pub struct DummyInterruptedTask;

    impl smeg_kernel::tasks::Task for DummyInterruptedTask { }

    impl<T> IsAddressableMut<T> for DummyInterruptedTask {
        fn is_addressable_mut(&self, _ptr: *mut T) -> bool { true }
    }

    pub struct IsrContext;

    impl smeg_kernel::interrupts::IsrContext for IsrContext { }

    impl HasInterruptedTask for IsrContext {
        type InterruptedTask = DummyInterruptedTask;

        fn interrupted_task(&self) -> Option<&Self::InterruptedTask> { None }
    }

    impl HasInterruptedTaskMut for IsrContext {
        fn interrupted_task_mut(&mut self) -> Option<&mut Self::InterruptedTask> { None }
    }
}

pub struct Drivers {
    mcu: mcu::Driver,
    syscall: syscall::Driver,
    blinky_blinky: blinky_blinky::Driver,
    task_scheduler: task_scheduler::Driver
}

pub struct IsrContext {
    mcu: mcu::IsrContext,
    syscall: syscall::IsrContext,
    blinky_blinky: blinky_blinky::IsrContext,
    task_scheduler: task_scheduler::IsrContext
}

impl smeg_kernel::interrupts::IsrContext for IsrContext { }

// TODO: all of these implementations probably ought to be tested... also candidates for a macro along with the above structs
impl AsRef<mcu::FamilyIsrContext> for IsrContext {
    fn as_ref(&self) -> &mcu::FamilyIsrContext { self.mcu.as_ref() }
}

impl AsMut<mcu::FamilyIsrContext> for IsrContext {
    fn as_mut(&mut self) -> &mut mcu::FamilyIsrContext { self.mcu.as_mut() }
}

impl From<mcu::FamilyIsrContext> for IsrContext {
    fn from(value: mcu::FamilyIsrContext) -> Self {
        Self {
            mcu: value.into(),
            syscall: syscall::IsrContext { }, // TODO: everything except the MCU should implement Default (or maybe From<...> ?)
            task_scheduler: task_scheduler::IsrContext { },
            blinky_blinky: blinky_blinky::IsrContext { }
        }
    }
}

impl HasTaskScheduler for IsrContext {
    type TaskScheduler = task_scheduler::IsrContext;
}

impl AsRef<task_scheduler::IsrContext> for IsrContext {
    fn as_ref(&self) -> &task_scheduler::IsrContext { &self.task_scheduler }
}

impl AsMut<task_scheduler::IsrContext> for IsrContext {
    fn as_mut(&mut self) -> &mut task_scheduler::IsrContext { &mut self.task_scheduler }
}

// TODO: temporary way to thrash out how a Driver should look whilst also verifying the code by blinking the LED on the Nucleo board
#[cfg(target_arch = "arm")]
mod blinky_blinky {
    use smeg_kernel::interrupts::HasIsrContext;

    use smeg_drivers_kernel_syscall::syscall_map;

    pub struct Dependencies;

    pub type Driver = super::super::blinky_blinky::Driver<Dependencies>;

    impl super::super::blinky_blinky::Dependencies for Dependencies {
        type IsrContext = super::IsrContext;
    }

    syscall_map! { BlinkyBlinkySyscall -> <Driver as super::super::blinky_blinky::Syscalls>::BlinkyBlinkySyscallHandler }

    pub type IsrContext = <Driver as HasIsrContext>::IsrContext;
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
        syscall::Driver::collect_isr_vectors(isrs)
    }
}
