use core::convert::{AsMut, AsRef, From};

use smeg_kernel::tasks::HasTaskScheduler;
use smeg_mcu_arm_cortex_m4_family::interrupts::IsrVectorTableBuilder as CortexM4IsrVectorTableBuilder;
use smeg_mcu_st_stm32l432kc::interrupts::IsrVectorTableBuilder as Stm32IsrVectorTableBuilder;

mod mcu {
    use smeg_kernel::interrupts::HasIsrContext;
    pub use smeg_mcu_st_stm32l432kc::import_driver;

    pub struct Dependencies;
    impl smeg_mcu_st_stm32l432kc::Dependencies for Dependencies { }

    pub type Driver = smeg_mcu_st_stm32l432kc::Driver<Dependencies>;

    pub type IsrContext = <Driver as HasIsrContext>::IsrContext;
    // pub type FamilyIsrContext = <Driver as HasFamilyIsrContext>::FamilyIsrContext; // TODO: DOES NOT WORK DUE TO 'impl From<T> for T' IN core !
    pub type FamilyIsrContext = smeg_mcu_arm_cortex_m4_family::interrupts::IsrContext; // But this DOES work despite being the same type... :-/
}

mod syscall {
    pub use smeg_drivers_kernel_syscall::import_driver;
    use smeg_kernel::interrupts::HasIsrContext;

    pub struct Dependencies;
    impl smeg_drivers_kernel_syscall::Dependencies for Dependencies {
        type IsrContext = super::IsrContext;
    }

    pub type Driver = smeg_drivers_kernel_syscall::Driver<Dependencies>;

    pub type IsrContext = <Driver as HasIsrContext>::IsrContext;
}

mod task_scheduler {
    pub use smeg_drivers_kernel_task_scheduler::import_driver;
    use smeg_kernel::IsAddressableMut;
    use smeg_kernel::tasks::{HasInterruptedTask, HasInterruptedTaskMut};

    pub struct Dependencies;
    impl smeg_drivers_kernel_task_scheduler::Dependencies for Dependencies {
        type IsrContext = super::IsrContext;
    }

    pub type Driver = smeg_drivers_kernel_task_scheduler::Driver<Dependencies>;

    // TODO: Temporary until a proper task scheduler driver is introduced
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

pub struct Drivers;

pub struct IsrContext {
    mcu: mcu::IsrContext,
    syscall: syscall::IsrContext,
    blinky_blinky: blinky_blinky::IsrContext,
    task_scheduler: task_scheduler::IsrContext
}

impl smeg_kernel::interrupts::IsrContext for IsrContext { }

macro_rules! collect_isr_vectors {
    ($isrs:ident) => { $isrs };

    ($driver:ident, $($drivers:ident),+) => { $driver::Driver::collect_isr_vectors(collect_isr_vectors!($($drivers),+)) };
}

// TODO: all of these implementations probably ought to be tested... also candidates for a macro along with the above structs
// TODO: something not quite right about this - the instantiation is good, it allows cache coherency for driver data, easy MMU mapping, etc.  The bad is the AsRef, which is only required for each driver to get its own (singleton) data, but could be used by other drivers that should be using the API on the IsrContext.  Maybe a way to get around this is to add something like a 'caller::XxxDriverOnly' to the AsRef method ?  And perhaps to other methods inside the XxxDriver ?
macro_rules! instantiate_drivers {
    ($driver:ident) => {
        impl AsRef<$driver::Driver> for IsrContext {
            fn as_ref(&self) -> &$driver::Driver {
                type Dependencies = $driver::Dependencies;

                #[unsafe(link_section = stringify!(".data.drivers.", $driver))]
                static DRIVER: $driver::Driver = $driver::import_driver!(Dependencies { });
                &DRIVER
            }
        }
    };

    ($driver:ident, $($drivers:ident),+) => {
        instantiate_drivers!($driver);
        instantiate_drivers!($($drivers),+);
    };
}

instantiate_drivers!(mcu, syscall, blinky_blinky, task_scheduler);

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

    pub struct Dependencies;

    pub type Driver = crate::blinky_blinky::Driver<Dependencies>;

    impl crate::blinky_blinky::Dependencies for Dependencies {
        type IsrContext = super::IsrContext;
    }

    pub use crate::blinky_blinky::import_driver;

    pub type IsrContext = <Driver as HasIsrContext>::IsrContext;
}

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
        collect_isr_vectors!(blinky_blinky, task_scheduler, syscall, isrs)
    }
}
