<!-- ANCHOR: module -->
This module is entirely and intentionally `unsafe`.  Even the bits that would normally be classed as safe in general Rust.  Here's why.

CPU and peripheral registers do not fit into Rust's model of the world.  They are volatile, mutable and global and cannot have a single owner.
Indeed, at a platform level, even interacting with 'normal' memory is rather `unsafe` due to architecture-specific constraints and semantics.

A simple example illustrates the problem of accessing some 'device memory' (MMIO) even in a uni-processor system; in a multi-processor system
there are further complications.  Assume there are two interrupts (`INT_A`, `INT_B`) with two priorities (`1`, `2`).  Our example MCU has a set
of flags for Interrupt ReQuest (IRQ) lines; these are single-bit Read-Write (RW) flags that are stored in a single memory-mapped register (ie. they
all fit into a single machine word).  An IRQ might be set by a hardware peripheral at any point in time - it is asynchronous to the CPU - and the
flag remains set until it is cleared by software (ie. the Interrupt Service Routine, the ISR).  The CPU also allows nested interrupts based on
priority, ie. pre-emption.  Nothing here is unusual for an MCU or CPU architecture.

A typical ISR template for our example architecture would look something like:
```pseudo
fn isr_x() -> ! {
    MCU_IRQ_FLAGS.clear_bit(IRQ_x); // Read-Modify-Write (RMW) operation
    do_isr_work();
    return_from_isr();
}
```
Suppose `isr_a()` is running at priority `1`.  In Rust, the RMW line in the above code snippet would need ownership (a mutable reference) to `MCU_IRQ_FLAGS`
in order to clear the `IRQ_A` flag to signal to the device that the interrupt has been handled.  But if `IRQ_B` is set and ISR `B` at priority `2`
pre-empts ISR `A` during this RMW operation, then it too requires ownership of `MCU_IRQ_FLAGS` to clear its `IRQ_B` flag.  ISRs cannot block otherwise
the system would deadlock.  The best we can hope for in this scenario is an atomic RMW operation provided by the hardware or created in software - a
single-cycle `bit clear` instruction or an LL / SC construct, for example.

The example above goes beyond IRQ flags and ISRs.  MCUs often group flags and registers from multiple peripherals and subsystems together, a scheme which
does not lend itself to partitioning access and ownership across different drivers when the lowest addressable unit is typically a byte.  Once multiple
cores are put into the mix then the problem just gets worse.  There is no getting away from the fact that there is _no single owner for global mutable
resources_.  Rust recognises this by making all `static mut` accesses `unsafe`.

Note that there is also another restriction in Rust that precludes references to volatile memory areas such as described in this example.  For volatile
accesses in Rust - which `MCU_IRQ_FLAGS` certainly is - we _cannot_ take _any_ reference to the volatile cell (eg. MMIO register) _anywhere in the
codebase_ due to Rust and LLVM code generation semantics (eg. the `dereferenceable` flag in LLVM, although that is not the only issue).  If a single
`&MCU_IRQ_FLAGS` or `&mut MCU_IRQ_FLAGS` exists in the codebase, which includes any methods that take `&self` or `&mut self`, then _the whole
application becomes Unsound_.  The reason for this is that volatile access invariants may be broken, such as by the compiler inserting multiple (or
speculative) reads to the volatile memory location.  We cannot even encapsulate accesses like [`core::ptr::volatile_read`] as methods on the register
bank's `struct` since the `&self` or `&mut self` reference is _Unsound_.  This obviously includes wrapping the `struct` with `UnsafeCell` or `MaybeUninit`.
If the volatile read has side-effects, for example reading from a FIFO buffer to consume an element is common for serial peripherals, then the program
will be _Unsound_ if there are any such references.  Much mirth and merriment will ensue whilst debugging such 'random' behaviour.  Fun Times for All.

For the reasons outlined above, the fancy ownership semantics and syntax provided by crates such as `volatile_cell` and `cortex_m` are _fundamentally
Unsound_ and cannot be used.  We are thus left managing access to the global mutable state such as MMIO registers and register banks using Rust- and
architecture-provided facilities, basically `unsafe` interfaces consisting of raw pointers, volatile accesses and inline assembly.

Another reason for exposing an entirely `unsafe` API is that, typically, these memory operations will be performed on system- and device-registers that
can impact, destabilise or brick the entire system.  Then there are issues like race conditions - instant _Undefined Behaviour_ in Rust - that can occur
if platform-specific access rules are not followed; for example, memory accesses that cross clock domains may require additional synchronisation or a
specific sequence of opcodes.

At the platform level, memory operations can _only_ be made safe by higher-level layers that can reason about encapsulation, isolation, mutual exclusion,
etc. so it is the responsibility of the MCU-specific crates and HAL drivers to provide safe abstractions for higher-level code - this includes class-level
drivers.  Indeed, this module is not intended for general consumption as it simply provides the primitives and building blocks but cannot provide a
Rust-safe or platform-safe interface.  Memory is hard.
<!-- ANCHOR_END: module -->

<!-- ANCHOR: MemoryAttributes -->
Marker trait for architecture-specific memory attributes.

Different architectures have different types of memory and different rules for interacting with those types of memory.  For example, some ARM
architectures may classify memory regions as _Normal Memory_, _Device Memory_, or _Strongly Ordered Memory_.  Each type will have its own specification
and semantics, with rules that implementors and software developers are expected to follow to keep system behaviour predictable and within specification.
An example might be that _Strongly Ordered Memory_ should not be read from or written to using an interruptible instruction, or maybe certain barriers or
fences are required to account for caching and speculative accesses when performing certain operations.

The purpose of the `MemoryAttributes` marker trait is to allow architectures to describe the memory in an architecture-specific way so that blanket
implementations of the memory operation traits can discriminate and specialise based on such attributes.
<!-- ANCHOR_END: MemoryAttributes -->

<!-- ANCHOR: Addressable -->
Traits common to all addressable blocks of memory.
<!-- ANCHOR_END: Addressable -->

<!-- ANCHOR: Bank -->
Trait for a Memory Bank, ie. a collection of [`Cell`]s.
<!-- ANCHOR_END: Bank -->

<!-- ANCHOR: Cell -->
Trait for an individually addressable unit of memory.
<!-- ANCHOR_END: Cell -->

<!-- ANCHOR: Readable -->
Marker trait for memory that can be read from.
<!-- ANCHOR_END: Readable -->

<!-- ANCHOR: Writable -->
Marker trait for memory that can be written to.
<!-- ANCHOR_END: Writable -->
