use std::mem::MaybeUninit;

// Needs to be visible to userspace clients and kernelspace
trait SequentialTypeId {
    fn sequential_type_id() -> usize;
}

/*
Note that this will _not_ work, since the static is hoisted out of the function and re-used for every T...
We can leverage #[derive(SequentialTypeId)] though...

impl<T: SyscallHandler> SequentialTypeId for T {
    fn sequential_type_id() -> usize {
        //#[unsafe(link_section = ".whatever.ids")]
        //#[unsafe(export_name = "can get this in a derive macro")]
        static ELF_SYMBOL: MaybeUninit<u8> = MaybeUninit::<u8>::new(0);
        &raw const ELF_SYMBOL as usize
    }
}
*/

// Needs to be visible to userspace clients and kernelspace
trait SyscallArgs: SequentialTypeId { }

// Needs to be visible to kernelspace only
#[allow(unused)]
struct SyscallContext<'isr, T: SyscallArgs> {
    pub args: &'isr mut T // can wrap this in MaybeUninit<> to force validation by the SyscallHandler
}

// Needs to be visible to kernelspace only
trait SyscallHandler {
    type Args: SyscallArgs;
    fn on_syscall<'isr>(&self, context: &mut SyscallContext<'isr, Self::Args>) -> Result<(), usize>;
}

// Needs to be visible to userspace clients and kernelspace
// Can implement the boilerplate with a #[derive(Syscall)]
struct SyscallA {
    _some_state: usize
}

impl SyscallArgs for SyscallA { }

impl SequentialTypeId for SyscallA {
    fn sequential_type_id() -> usize {
        //#[unsafe(link_section = ".whatever.ids")]
        //#[unsafe(export_name = "can get this in a derive macro")]
        static ELF_SYMBOL: MaybeUninit<u8> = MaybeUninit::<u8>::new(0);
        &raw const ELF_SYMBOL as usize
    }
}

// Needs to be visible to kernelspace only
struct SyscallHandlerA;

impl SyscallHandler for SyscallHandlerA {
    type Args = SyscallA;

    // Experiment with the lifetimes - can we elide them if they're specified directly in the ISR perhaps ?
    fn on_syscall<'isr>(&self, _context: &mut SyscallContext<'isr, Self::Args>) -> Result<(), usize> {
        println!("In SyscallHandler for A");
        Ok(())
    }
}

// Needs to be visible to userspace clients and kernelspace
struct SyscallB {
    _some_other_state: [u8; 3]
}

impl SyscallArgs for SyscallB { }

impl SequentialTypeId for SyscallB {
    fn sequential_type_id() -> usize {
        //#[unsafe(link_section = ".whatever.ids")]
        //#[unsafe(export_name = "can get this in a derive macro")]
        static ELF_SYMBOL: MaybeUninit<u8> = MaybeUninit::<u8>::new(0);
        &raw const ELF_SYMBOL as usize
    }
}

// Needs to be visible to kernelspace only
struct SyscallHandlerB;

impl SyscallHandler for SyscallHandlerB {
    type Args = SyscallB;

    fn on_syscall<'isr>(&self, _context: &mut SyscallContext<'isr, Self::Args>) -> Result<(), usize> {
        println!("In SyscallHandler for B");
        Ok(())
    }
}

// This can be a blanket implementation provided by the Syscall driver for whatever MCU it's compiled for:
trait SyscallInvocation {
    fn invoke_syscall(&mut self) -> Result<(), usize>;
}

impl<T: SyscallArgs> SyscallInvocation for T {
    fn invoke_syscall(&mut self) -> Result<(), usize> {
        // This will be the ARM 'svc' or something... r0 is T::sequential_type_id and r1 can be a pointer to self - the handler needs to verify that r1 (and r1 + sizeof<r1>) is within the stack boundaries, plus alignof<r1> == alignof<T> before it uses it...
        // HOWEVER - this can still introduce UB into the kernel if something passes a pointer to a block of uninitialised RAM, for example.  The alternative is to always get the arguments from a block stored in the TCB, but then this entails allowing the userspace code to access the TCB !  Neither is good, but the stack approach is better.  A third approach might be to use a static location known to both userspace and kernelspace (difficult, due to number of cores - no thread-local storage...) but that still doesn't address the underlying issue that the userspace code could write utter junk to the contents of the buffer (for example, a non-[0,1] in a bool field) and then the kernel ends up committing UB.  Maybe all SyscallArgs fields need to be MaybeUninit and each Syscall needs to validate all enum values, bool values, etc. ?  Probably the most sound approach and perhaps the only approach, but this puts the onus on any Syscall implementers.

        // The Cortex M4 crate can surround this with a #[cfg(not(feature = "no_default_syscall_invocation"))] to allow each MCU to define their own

        println!("Invoking syscall {}", T::sequential_type_id());
        let result = unsafe { syscall_isr(T::sequential_type_id(), self as *mut Self as usize) };
        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}

unsafe extern "C" fn syscall_isr(id: usize, args: usize) -> usize {
    // TODO: How do we get an efficient lookup based on ID and handler ?  A match cannot use non-const IDs.
    // Maybe a better way is to lean on the linker some more - put function pointers into a section demarcated by __LINKER_SYSCALL_TRAMPOLINE_START
    // and __LINKER_SYSCALL_TRAMPOLINE_PAST_END.  On syscall, put the pointer of the handler into r0.  The syscall ISR can subtract __LINKER_SYSCALL_TRAMPOLINE_START
    // and call the function pointer to do the dispatch.  This gets us the minimal table size and O(1) dispatch that is the goal, even if it is a bit
    // less idiomatic and means a bit of leakage of 'kernel stuff' into userspace, although that can be encapsulated / hidden.
    static SYSCALL_HANDLERS: (SyscallHandlerA, SyscallHandlerB) = (SyscallHandlerA, SyscallHandlerB);

    println!("ISR invoked for syscall {id:x} using args {args:x}");
    let result = if id == SyscallA::sequential_type_id() {
        // This is where the 'args fully on task stack' and alignof(args) needs to be checked (for each match arm), and then the args can be wrapped in a MaybeUninit to remind implementors of the handlers to do per-field validation
        let mut context = unsafe { SyscallContext { args: &mut *(args as *mut SyscallA) } };
        SYSCALL_HANDLERS.0.on_syscall(&mut context)

    } else if id == SyscallB::sequential_type_id() {
        let mut context = unsafe { SyscallContext { args: &mut *(args as *mut SyscallB) } };
        SYSCALL_HANDLERS.1.on_syscall(&mut context)
    } else {
        Err(123)
    };

    if result.is_ok() {
        0
    } else {
        result.unwrap_err()
    }
}

fn main() {
    assert!(SyscallA::sequential_type_id() != SyscallB::sequential_type_id());

    println!(
        "IDs = [A={:x}, B={:x}]",
        SyscallA::sequential_type_id(),
        SyscallB::sequential_type_id());

    let (mut a, mut b) = (
        SyscallA { _some_state: 0 },
        SyscallB { _some_other_state: [0; 3] }
    );

    _ = a.invoke_syscall();
    _ = b.invoke_syscall();
}
