use crate::caller;
use crate::bootstrapping::kernel::{BoardMcuBootstrapping/*, ContextSwitchingBootstrapping*/, McuCoreBootstrapping};
use crate::bootstrapping::rust::RuntimeBootstrapping;

pub unsafe trait Entrypoint {
    type RuntimeBootstrapper: RuntimeBootstrapping;
    type McuCoreBootstrapper: McuCoreBootstrapping;
    type BoardMcuBootstrapper: BoardMcuBootstrapping;
//    type Kernel: StaticRunnable<!>;

    unsafe fn entrypoint() -> ! {
        unsafe {
            if Self::McuCoreBootstrapper::mcu_core_id() == 0 {
                Self::RuntimeBootstrapper::bootstrap::<caller::IsKernel>();
            }
        }

        // TODO:
        // Where now...?  At this point we have the runtime initialised.  We want to reset the stack pointer and invoke the scheduler with the
        // kernel's initialisation task and pass in any injected types (eg. factories, the BoardMcuBootstrapper, etc.)
        #[cfg(test)] panic!("RuntimeBootstrapper was not called");

        #[cfg(target_arch = "arm")]
        unsafe {
            unsafe extern "C" { fn blinky_blinky() -> !; }
            blinky_blinky();
        }

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
// So...problem #1 (DONE) - figure out how to handle a panic _before_ runtime bootstrapping has completed, and problem #2, figure out how to handle injecting
// 'stuff' into the panic handler so it can figure out what core it's running on, what task is running, etc. etc.  Solving #2 allows us to recover or
// despair, depending on the (initialised) state of the system at the point when the panic occurred.



    // TODO: bootstrapping requirements to be added for the Cortex M4, before the first syscall:
    // 1. if there is an FPU, enable lazy stacking
    // 2. set the priorities of the SV_CALL and PENDSV interrupts to the lowest available a la ARM's approach in the reference manual

    // can tasks only become blocked through a syscall ?
    //
    // task switching can only be done through pendsv, otherwise tasks can be missed - eg. ISRs A, B, C where priority 'A > B > C'.  If B is running and
    // sets the task context to its preferred task, then if it is pre-empted by 'A' after that but before returning, which also sets the task context to
    // its preferred task, then 'B's task will not get run and will be 'lost'.  We need to ensure that ISRs only flag their preferred tasks as 'runnable'
    // or clear a bitflag or something _at the same time as setting the pendsv flag_ so that the pendsv can pick the highest priority task, although this
    // falls down if there is more than one at the same priority. a scheduler can figure out what needs running based on priorities... however this will
    // require three context switches...one into the scheduler...one back into the syscall / pendsv...and finally into the task.
    //
    // we need a way to allow higher priority tasks to pre-empt whatever is running and short-circuit the scheduler, for example to allow a high-priority
    // interrupt to exit the ISR quickly and transfer control into its (presumably high-priority) processing task, reducing context switching overhead.
    //
    // to request a context switch, all 'ready' tasks must be in the prioritised queue, then set icsr.pendsvset
    //
    // just before searching for and picking the highest priority task from the queue, set icsr.pendsvclr - if a higher-priority interrupt sets
    // icsr.pendsvset after this then the algorithm will run again (at the expense of discarding the result of the context-switch and doing another one),
    // but nothing is lost.  Before the _actual_ context switching (setting the PSP, MMU, etc. etc.) then we could re-check icsr.pendsvset and re-search
    // the queues if necessary, ie. a loop:
    // do {
    //   icsr.pendsvclr = 1;
    //   find highest priority task - probably involves locking a shared list ?
    // } while (icsr.pendsvset)
    // switch to highest priority task && return

    // All ISRs (not just SV_CALL) will need to return via the EXC_RETURN mechanism (leverage 'bx' in some inline assembly, for example) - the trampoline
    // takes care of this by setting LR prior to calling the target.  Prior to the syscall ISR returning, the caller-saved r1 value in the stack needs to
    // be overwritten with the usize representation of SyscallResult, so that when the CPU pops r1 on return, the caller can use it as a SyscallResult.  The
    // use of r1 over r0 allows the caller to determine which r0 (syscall ID) triggered the error, since a SyscallErrorCode is not wide enough



        loop { }

// Drivers need a way to register an ISR (at compile-time):
//     Probably best to use an MCU-provided macro to define the ISRs to ensure the ABI, signature, prologue, epilogue are correct
//     We need provision for raw ISRs as well as per-core ISRs that take a given context as an argument
//     The MCU-specific macro can decide how to implement the ISR, probably as a specially named function or a const-friendly Some(...).or(...) type thing
//     All ISRs should be disabled
//
//     Concept:
//         #[derive(PerCoreIsr)]
//         struct SomeState {
//             ...
//         }
//
//         impl PerCoreIsr for SomeState ?

// Bootstrap the syscall mechanism - this involves:
//     The syscall mechanism is just a driver, so the per-core ISR should've already been registered at compile-time
//     Any initialisation code needs to be run, such as enabling the syscall interrupt after all other initialisation has been done
//     As part of building up the ISR body, the syscall driver needs to examine all other drivers for exported syscalls that can be dispatched to
//     Each syscall ought to be able to be enabled and disabled - calling disabled syscalls ought to kernel_panic!(...) with a TaggedError
//     The panic_handler, if called from within an ISR because of a failed syscall, ought to flag the offending task as 'not runnable / panicked' and
//     schedule a user-space recovery task for the bad task, to avoid staying in an ISR too long

// Bootstrap the context-switching (yield) syscall - this involves:
//     calling back to the MCU module and passing a kernel task to continue system initialisation
//     note that kernel tasks cannot be pre-empted by other kernel tasks unless we switch to a pure ISR stack and use separate stacks for each kernel task
//     (leave this option open - probably the better design)
//
// context-switching bootstrapping for the Cortex M4 is going to:
//     set the PSP stack pointer to the top of the kernel task's
//     reset the MSP stack pointer back to the top for ISRs to use
//     enable interrupts, preferably just the syscall one
//     revoke kernel privileges
//     return from interrupt (but the start of the kernel task)

// A driver can be provided for the trap exception for illegal instructions, privilege issues, etc. which involves:
//     default implementation will just despair or reset the core (feature-toggled)
//     any number of driver implementations can be provided to do more interesting things, such as terminate the offending tasks, reset the MCU, emulate
//     hardware / missing instructions, etc.

// Note that per-core data should be placed adjacent to the per-core ISR stack when linking.  This ensures locality and also an easier way to mark a
// contiguous block of RAM as protected / accessible for a given core if there is an MPU available.


        // We need the MCU to reset the SP to the top of the core's stack, then jump to an endpoint we give it so we can create an object and call into that.
        // Maybe need a SchedulerBootstrapping, which does that...

        // kernel
        //     - syscall mechanism; needs to be (compile-time-)extensible for drivers to create their own syscalls (the scheduler is a driver...)
        //     - scheduler bootstrapping
        //     - context switching; needs to be (compile-time-)extensible for drivers to save / restore their own per-task state; only tasks using the driver, though

        // set msp = top of current core's stack and branch to entrypoint
        // entrypoint creates syscall
        // entrypoint creates scheduler on stack (factory for this needs to be injected)
        // scheduler is
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
    fn entrypoint__called_when_mcu_core_id_is_zero__expect_runtime_bootstrapper_is_called() {
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
    fn entrypoint__called_when_mcu_core_id_is_not_zero__expect_runtime_bootstrapper_is_not_called() {
        _entrypoint__called_when_mcu_core_id_is_not_zero__expect_runtime_bootstrapper_is_not_called::<1>();
        _entrypoint__called_when_mcu_core_id_is_not_zero__expect_runtime_bootstrapper_is_not_called::<2>();
        _entrypoint__called_when_mcu_core_id_is_not_zero__expect_runtime_bootstrapper_is_not_called::<345>();
    }

    fn _entrypoint__called_when_mcu_core_id_is_not_zero__expect_runtime_bootstrapper_is_not_called<const MCU_CORE_ID: usize>() {
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
