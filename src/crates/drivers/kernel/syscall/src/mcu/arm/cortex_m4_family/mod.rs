use core::borrow::BorrowMut;

use smeg_kernel::interrupts::IsrContext;
use smeg_kernel::bootstrapping::kernel::IsrBootstrapping;
use smeg_kernel::syscalls::SyscallResult;

use smeg_mcu_arm_cortex_m4_family::isr_fn_trampolines;
use smeg_mcu_arm_cortex_m4_family::interrupts::{HasIsrBasicStackFrameMut, IsrContextImpl, IsrVectorTableBuilder};

isr_fn_trampolines! {
    fn on_sv_call_isr_trampoline() -> on_sv_call_isr<>() -> "thread_main" /* TODO: "thread_process" or even a new option, to allow context-switching */;
}

pub const fn collect_isr_vectors<I>(isrs: IsrVectorTableBuilder<I>) -> IsrVectorTableBuilder<I>
    where
        I: IsrBootstrapping,
        I::IsrContext: IsrContext + From<IsrContextImpl> + BorrowMut<IsrContextImpl> {

    IsrVectorTableBuilder::<I> {
        sv_call: Some(on_sv_call_isr_trampoline::<I::IsrContext>),
        ..isrs
    }
}

unsafe fn on_sv_call_isr<C: BorrowMut<IsrContextImpl>>(isr_context: &mut C) {
    // TODO: temporary blinky-blinky stuff, to verify syscall invocation on the Nucleo board...

    unsafe {
        let stack_frame = isr_context.borrow_mut().basic_stack_frame_mut();

        static mut ODR: *mut usize = (0x48000400_usize + 20_usize) as *mut usize;
        let mut odr = core::ptr::read_volatile(ODR);
        if stack_frame.r0 == 1 {
            odr |= 1 << 3;
        } else if stack_frame.r0 == 0 {
            odr &= !(1 << 3);
        }

        core::ptr::write_volatile(ODR, odr);

        stack_frame.r1 = 0;
    }


    // just the Cortex-specific bits here, then call into a more generic function that returns a SyscallResult
    // specifically, extract the value of r0 to use as the syscall ID and call the generic handler
    // we rely on the fact that the CPU's context-saving uses the same ABI as C functions, so the Rust compiler will 'do the right thing' with any extra
    // registers that need pushing to the stack.  Somewhere in the bootstrapping, the lazy FPU context-saving also would have been set, so if any drivers
    // utilise FPU instructions then the context-saving should happen automatically, or during PENDSV task switching we can force it.  Basically, at this
    // point, we should be pretty 'safe' for high-level code generation by Rust, with some caveats around the return value (below).

    // the generic syscall handler, using the "Rust" ABI, can then...
    // examine the Task Control Block, which can have a bunch of syscall-specific storage slots for richer argument-passing, already set by the caller prior
    // to the 'svc' opcode
    // if there is no Task Control Block then we need to return an Err(...)
    // get current core ID (TODO: how to determine without having access to a HasMcuCoreId, as injected into the kernel by the Composition Root ?)
    // get registered syscalls - if unknown syscall then return with Err(SyscallErrorCode::UnknownSyscall)
    // call corresponding registered handler with its core-specific context
    // return a SyscallResult back to this ISR, which can update the 'stack_frame.r1' with the equivalent usize before returning



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
/*
    smeg_kernel::despair!(
        with(smeg_kernel::errors::KernelErrorCode::GeneralDespair(0)),
        because("TODO: the unrecognised syscall can actually return an Err(SyscallErrorCode::UnknownSyscall)..."));
*/
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use fluent_test::prelude::*;

    use smeg_mcu_arm_cortex_m4_family::interrupts::test_doubles::{Dummy, Stub};

    use super::*;

    #[unsafe(no_mangle)]
    unsafe extern "C" fn _reset_handler() -> ! {
        panic!("Aborting because the _reset_handler stub should never be called");
    }

    type IsrVectorTableBuilder = super::IsrVectorTableBuilder<Dummy>;

    #[test]
    fn collect_isr_vectors__called__expect_same_vectors_excluding_sv_call() {
        let original_isrs = IsrVectorTableBuilder::from(Stub);
        let original_isrs_excluding_sv_call = IsrVectorTableBuilder { sv_call: None, ..original_isrs };

        let added_isrs = collect_isr_vectors(original_isrs.clone());
        let added_isrs_excluding_sv_call = IsrVectorTableBuilder { sv_call: None, ..added_isrs };

        expect!(added_isrs_excluding_sv_call == original_isrs_excluding_sv_call).to_be_true();
    }

    #[test]
    fn collect_isr_vectors__called__expect_sv_call_isr_is_added() {
        let original_isrs = IsrVectorTableBuilder::from(Stub);
        let added_isrs = collect_isr_vectors(original_isrs.clone());
        expect!(added_isrs.sv_call).to_equal(Some(on_sv_call_isr_trampoline::<Dummy>));
    }
}
