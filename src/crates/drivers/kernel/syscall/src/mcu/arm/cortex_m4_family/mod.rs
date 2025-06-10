use smeg_kernel::syscalls::SyscallResult;

pub use smeg_mcu_arm_cortex_m4_family::interrupts::IsrVectorTableBuilder;

pub const fn collect_isr_vectors(isrs: IsrVectorTableBuilder) -> IsrVectorTableBuilder {
    IsrVectorTableBuilder {
        sv_call: Some(on_sv_call_isr),
        ..isrs
    }
}

// TODO: Because of pre-emptive late-arriving exceptions not pushing context of their own (ARMv7-M reference manual, see B1.5.11 and 'ExceptionTaken()' on
// page B1-589), the values of r0-r3 are UNKNOWN on entry !  This means that we cannot rely on the function-call syntax, which would be awesome and
// efficient - instead, we need to examine the value of r0 from the stack :(  Update this function signature accordingly...
unsafe extern "C" fn on_sv_call_isr(_r0: usize, _r1: usize, _r2: usize, _r3: usize) -> ! {
    // just the Cortex-specific bits here, then call into a more generic function that returns a SyscallResult
    // specifically, extract the value of r0 to use as the syscall ID and call the generic handler
    // we rely on the fact that the CPU's context-saving uses the same ABI as C functions, so the Rust compiler will 'do the right thing' with any extra
    // registers that need pushing to the stack.  Somewhere in the bootstrapping, the lazy FPU context-saving also would have been set, so if any drivers
    // utilise FPU instructions then the context-saving should happen automatically.  Basically, at this point, we should be pretty 'safe' for high-level
    // code generation by Rust, with some caveats around the return value (below).

    // the generic syscall handler, using the "Rust" ABI, can then...
    // examing the Task Control Block, which can have a bunch of syscall-specific storage slots for argument-passing, already set by the caller
    // if there is no Task Control Block then we need to return an Err(...)
    // get current core ID (TODO: how to determine without having access to a HasMcuCoreId, as injected into the kernel by the Composition Root ?)
    // get registered syscalls - if unknown syscall then return with Err(SyscallErrorCode::UnknownSyscall)
    // call corresponding registered handler with its core-specific context
    // return a SyscallResult back to this ISR

    // All ISRs (not just SV_CALL) will need to return via the EXC_RETURN mechanism (leverage 'bx' in some inline assembly, for example).  Prior to returning,
    // the caller-saved r1 value in the stack needs to be overwritten with the usize representation of SyscallResult, so that when the CPU pops r1 on return,
    // the caller can use it as a SyscallResult.  The use of r1 over r0 allows the caller to determine which r0 (syscall ID) triggered the error, since a
    // SyscallErrorCode is not wide enough

    smeg_kernel::despair!(
        with(smeg_kernel::errors::KernelErrorCode::GeneralDespair(0)),
        because("TODO: the unrecognised syscall can actually return an Err(SyscallErrorCode::UnknownSyscall)..."));
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use fluent_test::prelude::*;

    use smeg_kernel::test_doubles::Stub;

    use super::*;

    #[unsafe(no_mangle)]
    unsafe extern "C" fn _reset_handler() -> ! {
        panic!("Aborting because the _reset_handler stub should never be called");
    }

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
        expect!(added_isrs.sv_call).to_equal(Some(on_sv_call_isr));
    }
}
