use std::mem::{align_of, MaybeUninit};

// Boilerplate for implementing drivers, so lives in the syscall driver crate
type SyscallIsrTrampolinePtr = unsafe fn(args: usize) -> Result<(), usize>;

// Boilerplate for implementing drivers, so lives in the syscall driver crate
unsafe trait SyscallIsrTrampoline {
    unsafe fn on_syscall(args: usize) -> Result<(), usize>;
}

// Needs to be visible to userspace clients and kernelspace; lives in the syscall driver crate
trait HasSyscallId {
    fn syscall_id() -> usize;
}

// Needs to be visible to userspace clients and kernelspace; lives in the syscall driver crate
trait SyscallArgs: HasSyscallId { }

// Needs to be visible to any crates that define SyscallHandlers, so needs to live in the syscall driver crate
unsafe impl<H: SyscallHandler> SyscallIsrTrampoline for H {
    unsafe fn on_syscall(args: usize) -> Result<(), usize> {
        // Alignment can be checked here; the size can too if the IsrContext is passed in so we can retrieve the current task's stack
        // (or heap) information, which will be the case in the actual implementation.
        if align_of::<H::Args>() > 1 && args & (align_of::<H::Args>() - 1) != 0 {
            return Err(123); // Some sort of error code for an unaligned access
        }

        let mut context = unsafe { SyscallContext { args: &mut *(args as *mut MaybeUninit<H::Args>) } };
        <H as SyscallHandler>::on_syscall(&mut context)
    }
}

// Needs to be visible to kernelspace only; lives in the syscall driver crate.  The #[allow(unused)] is because we don't use it for anything in
// the Poc.  The MaybeUninit<T> is important because beyond alignment and address constraints, we don't even know if it's of valid construction.
// The visibility can also be changed - the struct is just an example.  ISR context will also need to be provided (to retrieve the MCU core, etc.)
#[allow(unused)]
struct SyscallContext<'isr, T: SyscallArgs> {
    pub args: &'isr mut MaybeUninit<T>
}

// Needs to be visible to kernelspace only; lives in the syscall driver crate
trait SyscallHandler {
    type Args: SyscallArgs;

    fn on_syscall(context: &mut SyscallContext<Self::Args>) -> Result<(), usize>; // The usize will actually be a KernelError; we want a SyscallResult
}

// Needs to be visible to userspace clients and kernelspace; lives in the crate for whatever driver is defining it
// Can implement the SyscallArgs trait boilerplate with a #[derive(Syscall)]
// #[derive(Syscall)]
struct SyscallA {
    _some_state: usize
}

impl SyscallArgs for SyscallA { }

impl HasSyscallId for SyscallA {
    fn syscall_id() -> usize {
        // Since this is going to be implemented inside a #[derive(Syscall)], we can extract the name of the struct and substitute that into the name
        // of the static so that we have a well-known symbol name that can easily be made unique for each Syscall type.  Note that the 'extern' also
        // allows any number of handler implementations (as long as there is only one compiled), so common Syscalls can be defined in, say, a HAL
        // driver and hardware-specific implementations of the SyscallHandlers can be in different crates, which is why we do not want the definition
        // here / it belongs with the handler.
        unsafe extern "Rust" { static __SYSCALL_HANDLER_A: SyscallIsrTrampolinePtr; }
        &raw const __SYSCALL_HANDLER_A as usize
    }
}

// Needs to be visible to kernelspace only; lives in the crate for whatever driver is providing the implementation
// #[SyscallHandler]
struct SyscallHandlerA;

const _: () = {
    // Implementation detail, which can be encapsulated by #[SyscallHandler] to ensure the correct naming, etc.
    #[unsafe(no_mangle)]
    // #[unsafe(link_section = ".whatever.syscalls.trampolines")]
    static __SYSCALL_HANDLER_A: SyscallIsrTrampolinePtr = <SyscallHandlerA as SyscallIsrTrampoline>::on_syscall;
};

// Defining the handler - user-provided implementation living in the crate for whatever driver is providing it
impl SyscallHandler for SyscallHandlerA {
    type Args = SyscallA;

    // A static (associated function) - from here, this could be turned into a static instance of SyscallHandlerA, a local instance, no instance, etc.
    // How the implementation looks, what state it stores (static or per-core) or whether it is mutable, or in what linker section, is all up to the driver.
    fn on_syscall(_context: &mut SyscallContext<Self::Args>) -> Result<(), usize> {
        // Note that context.args is inside MaybeUninit<Args> and needs thoroughly validating.  For some syscalls this may be un-necessary (eg. if they
        // are ZSTs), but for others this will entain checking boolean fields are 0 or 1, pointers are in the correct address space, etc.
        println!("In SyscallHandler for A");
        Ok(())
    }
}

// Needs to be visible to userspace clients and kernelspace
// #[derive(Syscall)]
struct SyscallB {
    _some_other_state: [u8; 3]
}

impl SyscallArgs for SyscallB { }

impl HasSyscallId for SyscallB {
    fn syscall_id() -> usize {
        unsafe extern "Rust" { static __SYSCALL_HANDLER_B: SyscallIsrTrampolinePtr; }
        &raw const __SYSCALL_HANDLER_B as usize
    }
}

// Needs to be visible to kernelspace only
// #[SyscallHandler]
struct SyscallHandlerB;

const _: () = {
        #[unsafe(no_mangle)]
        // #[unsafe(link_section = ".whatever.syscalls.trampolines")]
        pub static __SYSCALL_HANDLER_B: SyscallIsrTrampolinePtr = <SyscallHandlerB as SyscallIsrTrampoline>::on_syscall;
};

impl SyscallHandler for SyscallHandlerB {
    type Args = SyscallB;

    fn on_syscall(_context: &mut SyscallContext<Self::Args>) -> Result<(), usize> {
        println!("In SyscallHandler for B");
        Ok(())
    }
}

// This will be built up by the linker, by collecting all trampolines since they are all in the same section.  In practice it will
// be two pointers, something like __LINKER_SYSCALLS_TRAMPOLINES_START and __LINKER_SYSCALLS_TRAMPOLINES_PAST_END, but we're using
// an array for simplicity:
// static __LINKER_SYSCALLS_ISR_TRAMPOLINES: [SyscallIsrTrampolinePtr; 2] = [__SYSCALL_HANDLER_A, __SYSCALL_HANDLER_B];
/*
extern static __LINKER_SYSCALLS_TRAMPOLINES_START: MaybeUninit<SyscallIsrTrampolinePtr>;
extern static __LINKER_SYSCALLS_TRAMPOLINES_PAST_END: MaybeUninit<SyscallIsrTrampolinePtr>

const _: () = {
    assert!((&raw const __LINKER_SYSCALLS_TRAMPOLINES_START as usize) & (align_of::<SyscallIsrTrampolinePtr>() - 1) == 0, "something something unaligned");
    assert!((&raw const __LINKER_SYSCALLS_TRAMPOLINES_PAST_END as usize) & (align_of::<SyscallIsrTrampolinePtr>() - 1) == 0, "more unalignment");
}
*/

// This can be a blanket implementation provided by the Syscall driver for whatever MCU it's compiled for, so it needs to live in the syscall driver crate:
trait SyscallInvocation {
    fn invoke_syscall(&mut self) -> Result<(), usize>;
}

impl<T: SyscallArgs> SyscallInvocation for T {
    #[inline(always)]
    fn invoke_syscall(&mut self) -> Result<(), usize> {
        // This will be the ARM 'svc' or something... r0 is T::syscall_id and r1 can be a pointer to self - the handler needs to verify
        // that r1 (and r1 + sizeof<r1>) is within the stack boundaries, plus alignof<r1> == alignof<T> before it uses it...
        // HOWEVER - this can still introduce UB into the kernel if something passes a pointer to a block of uninitialised RAM, for example.  The
        // alternative is to always get the arguments from a block stored in the TCB, but then this entails allowing the userspace code to access
        // the TCB !  Neither is good, but the stack approach is better.  A third approach might be to use a static location known to both userspace
        // and kernelspace (difficult, due to number of cores - no thread-local storage...) but that still doesn't address the underlying issue that
        // the userspace code could write utter junk to the contents of the buffer (for example, a non-[0,1] in a bool field) and then the kernel
        // ends up committing UB.  Maybe all SyscallArgs fields need to be MaybeUninit and each Syscall needs to validate all enum values, bool
        // values, etc. ?  Probably the most sound approach and perhaps the only approach, but this (correctly) puts the onus on any Syscall
        // implementers - we just need to provide adequate signposting and explicitness when passing around 'stuff' that originated in userspace.

        // The Cortex M4 crate can surround this with a #[cfg(not(feature = "no_default_syscall_invocation"))] to allow each MCU to define their own
        // implementation if there is something special that needs to be taken into account.

        // Keep the implementation as small as possible with pretty much no error checking.  Syscalls will be invoked many times (many callsites) and the
        // fastest and smallest implementation will be perhaps three or four inlined assembly instructions, which will be comparable to an actual function
        // call sequence and also avoids stack frame overhead when not inlining.  Note that the onus for error checking is on the handler that runs in
        // privileged mode, so any checking here is just superfluous bloat for the inlined calls.

        println!("Invoking syscall {:x}", T::syscall_id());
        let result = unsafe { syscall_isr(T::syscall_id(), self as *mut Self as usize) };
        if result == 0 {
            println!("Syscall {:x} invoked successfully !", T::syscall_id());
            Ok(())
        } else {
            println!("OOPS !  Syscall {:x} returned error {} !", T::syscall_id(), result);
            Err(result)
        }
    }
}

// The ISR obviously lives in the syscall driver crate.  The implementation will be identical across architectures, since the ISR trampoline will have
// the platform-specific marshalling...
unsafe extern "C" fn syscall_isr(id: usize, args: usize) -> usize {
    println!("ISR invoked for syscall {id:x} using args {args:x}");
    if id & (align_of::<SyscallIsrTrampolinePtr>() - 1) != 0 {
        return 456; // The address might be within range, but it's not a valid function pointer
    }

    assert!(size_of::<SyscallIsrTrampolinePtr>() == align_of::<SyscallIsrTrampolinePtr>(), "we've made the assumption that size and alignment are the same (ie. a single field) to allow a quick check (ie. single comparison), otherwise it is possible to have correct alignment of something that doesn't point to the start of the struct, which may not be a simple power-of-two with AND mask test...");

    let trampoline = id as *const SyscallIsrTrampolinePtr; // UB only happens on dereferencing the pointer so we're able to check its bounds first
/*
    Further ID validation will be something along the lines of...

    if
        (trampoline < &raw const __LINKER_SYSCALLS_ISR_TRAMPOLINES_START) ||
        (trampoline > (&raw const __LINKER_SYSCALLS_ISR_TRAMPOLINES_PAST_END).offset(-1)) {

        return 789; // The address (ID) is out of range, so not a valid function pointer
    }

    But for the PoC we'll be using...
*/
    unsafe extern "Rust" { static __SYSCALL_HANDLER_A: SyscallIsrTrampolinePtr; static __SYSCALL_HANDLER_B: SyscallIsrTrampolinePtr; }
    if trampoline != &raw const __SYSCALL_HANDLER_A && trampoline != &raw const __SYSCALL_HANDLER_B {
        return 789;
    }

    let result = unsafe { (*trampoline)(args) };
    if result.is_ok() {
        0
    } else {
        result.unwrap_err()
    }
}

fn main() {
    assert!(SyscallA::syscall_id() != SyscallB::syscall_id());

    println!(
        "IDs = [A={:x}, B={:x}]",
        SyscallA::syscall_id(),
        SyscallB::syscall_id());

    let (mut a, mut b) = (
        SyscallA { _some_state: 0 },
        SyscallB { _some_other_state: [0; 3] }
    );

    _ = a.invoke_syscall();
    _ = b.invoke_syscall();

    println!("Invoking syscall to do bad things...");
    unsafe extern "Rust" { static __SYSCALL_HANDLER_A: SyscallIsrTrampolinePtr; }
    let result = unsafe { syscall_isr(&raw const __SYSCALL_HANDLER_A as usize + 1, 0) };
    println!("Result is {result}");
}
