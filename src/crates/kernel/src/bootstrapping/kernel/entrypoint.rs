use crate::caller;
use crate::bootstrapping::kernel::{BoardMcuBootstrapping, McuCoreBootstrapping};
use crate::bootstrapping::rust::RuntimeBootstrapping;

pub unsafe trait Entrypoint {
    type RuntimeBootstrapper: RuntimeBootstrapping;
    type McuCoreBootstrapper: McuCoreBootstrapping;
    type BoardMcuBootstrapper: BoardMcuBootstrapping;
//    type Kernel: StaticRunnable<!>;

    unsafe fn entrypoint() -> ! {
        unsafe {
            if Self::McuCoreBootstrapper::core_id() == 0 {
                Self::RuntimeBootstrapper::bootstrap::<caller::IsKernel>();
            }
        }

        // TODO:
        // Where now...?  At this point we have the runtime initialised.  We want to reset the stack pointer and invoke the scheduler with the
        // kernel's initialisation task and pass in any injected types (eg. factories, the BoardMcuBootstrapper, etc.)
        #[cfg(test)] panic!("RuntimeBootstrapper was not called");

// Maybe something like this comes next:
// McuCoreBootstrapper::bootstrap::<Kernel>()
//
// Note that we need some form of TaskControlBlock setting up early on, so any panic!(...) can determine the course of action.  If, for example, the
// task that panics is the startup (maybe in this case Option<TaskControlBlock> = None, _for the current core_) then we should despair, otherwise we
// can reclaim (Drop) the resources held by the task.
//
// The tricky bit is getting information injected into the panic handler that allows it to determine _the current core_, since the task control block
// is going to be a per-core setting - it's not enough for the #[panic_handler] to decorate only one instance of a function, it specifically needs to
// decorate a free function, so leveraging generics (structs or traits) is not possible.  Perhaps we need a function pointer, initialised as a
// static - this is doable _iff_ the runtime bootstrapper never panics...but it can, since calculating offset pointers, etc. via core::ptr asserts !
//
// So...problem #1 - figure out how to handle a panic _before_ runtime bootstrapping has completed, and problem #2, figure out how to handle injecting
// 'stuff' into the panic handler so it can figure out what core it's running on, what task is running, etc. etc.  Solving #2 allows us to recover or
// despair, depending on the (initialised) state of the system at the point when the panic occurred.

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
        type PanicBootstrapper = Dummy;

        unsafe fn bootstrap<K: caller::RestrictedToKernel>() {
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
