use crate::HasMcuCoreId;
use crate::bootstrapping::kernel::{BoardMcuBootstrapping, McuCoreBootstrapping};
use crate::bootstrapping::rust::RuntimeBootstrapping;

pub unsafe trait Entrypoint {
    type RuntimeBootstrapper: RuntimeBootstrapping;
    type McuCoreBootstrapper: McuCoreBootstrapping;
    type BoardMcuBootstrapper: BoardMcuBootstrapping;

    unsafe fn entrypoint() -> ! {
        unsafe {
            if Self::McuCoreBootstrapper::core_id() == 0 {
                Self::RuntimeBootstrapper::bootstrap();
            }
        }

        // TODO:
        // Where now...?  At this point we have the runtime initialised.  We want to reset the stack pointer and invoke the scheduler with the
        // kernel's initialisation task and pass in any injected types (eg. factories, the BoardMcuBootstrapper, etc.)
        #[cfg(test)] panic!("RuntimeBootstrapper was not called");

        loop { }

        // We need the MCU to reset the SP to the top of the core's stack, then jump to an endpoint we give it so we can create an object and call into that.
        // Maybe need a SchedulerBootstrapping, which does that...
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::{io, thread};
    use std::borrow::Cow;

    use fluent_test::prelude::*;

    use crate::test_doubles::Dummy;
    use crate::bootstrapping::kernel::test_doubles::mcu_core_bootstrapping::StubForConstantMcuCoreId;

    use super::*;

    struct StubRuntimeBootstrapperForPanic;
    unsafe impl RuntimeBootstrapping for StubRuntimeBootstrapperForPanic {
        type BssSectionInitialiser = Dummy;
        type DataSectionInitialiser = Dummy;
        type McuMemoryBootstrapper = Dummy;

        unsafe fn bootstrap() {
            panic!("RuntimeBootstrapper was called");
        }
    }

    #[test]
    fn entrypoint__called_when_core_id_is_zero__expect_runtime_bootstrapper_is_called() {
        struct StubEntrypoint;
        unsafe impl Entrypoint for StubEntrypoint {
            type RuntimeBootstrapper = StubRuntimeBootstrapperForPanic;
            type McuCoreBootstrapper = StubForConstantMcuCoreId<0>;
            type BoardMcuBootstrapper = Dummy;
        }

        let result = &*result_from_running::<StubEntrypoint>();
        expect!(result).to_equal("RuntimeBootstrapper was called");
    }

    fn result_from_running<'a, T: Entrypoint>() -> Cow<'a, str> {
        use smeg_testing_host_utils::threads::PanicReason;
        thread::scope(|scope| -> io::Result<Cow<'a, str>> {
            let result = thread::Builder::new().spawn_scoped(scope, || {
                unsafe { T::entrypoint(); }
                #[allow(unreachable_code)] { unreachable!("Entrypoint is not expected to return"); }
            })?.join().panic_reason();

            Ok(result.unwrap_or_else(|| Cow::from("This should never happen - the only way to exit the entrypoint is panic!(...)")))
        }).expect("entrypoint must run successfully")
    }

    #[test]
    fn entrypoint__called_when_core_id_is_not_zero__expect_runtime_bootstrapper_is_not_called() {
        _entrypoint__called_when_core_id_is_not_zero__expect_runtime_bootstrapper_is_not_called::<1>();
        _entrypoint__called_when_core_id_is_not_zero__expect_runtime_bootstrapper_is_not_called::<2>();
        _entrypoint__called_when_core_id_is_not_zero__expect_runtime_bootstrapper_is_not_called::<345>();
    }

    fn _entrypoint__called_when_core_id_is_not_zero__expect_runtime_bootstrapper_is_not_called<const MCU_CORE_ID: usize>() {
        struct StubEntrypoint<const MCU_CORE_ID: usize>;
        unsafe impl<const MCU_CORE_ID: usize> Entrypoint for StubEntrypoint<MCU_CORE_ID> {
            type RuntimeBootstrapper = StubRuntimeBootstrapperForPanic;
            type McuCoreBootstrapper = StubForConstantMcuCoreId<MCU_CORE_ID>;
            type BoardMcuBootstrapper = Dummy;
        }

        let result = &*result_from_running::<StubEntrypoint<MCU_CORE_ID>>();
        expect!(result).to_equal("RuntimeBootstrapper was not called");
    }
}
