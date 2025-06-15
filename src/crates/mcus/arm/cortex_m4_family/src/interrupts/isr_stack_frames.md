<!-- ANCHOR: IsrBasicStackFrame -->
Layout of a Basic Stack Frame at the time of Exception / Interrupt entry, as per Section B1.5.7 of the ARMv7-M Architecture Reference Manual.

The Basic Stack Frame is always available and at the same position on the stack no matter how the context-saving has been configured or aligned, which makes it relatively cheap and easy to use.

Note that the values of the registers do not necessarily correspond to the values of the stacked registers because the Cortex M4 architecture allows late-arriving higher-priority exceptions to pre-empt the exception that caused the context saving.  If the higher priority exception arrives before the initial exception starts executing then there is no need to push further state onto the stack, but this does mean that, for example, the `SV_CALL` exception handler needs to examine the contents of the stack for its arguments rather than rely on the register values on entry.
<!-- ANCHOR_END: IsrBasicStackFrame -->

<!-- ANCHOR: IsrBasicStackFrame.r0 -->
The value of register `r0` pushed onto the stack at the time of context saving.
<!-- ANCHOR_END: IsrBasicStackFrame.r0 -->

<!-- ANCHOR: IsrBasicStackFrame.r1 -->
The value of register `r1` pushed onto the stack at the time of context saving.
<!-- ANCHOR_END: IsrBasicStackFrame.r1 -->

<!-- ANCHOR: IsrBasicStackFrame.r2 -->
The value of register `r2` pushed onto the stack at the time of context saving.
<!-- ANCHOR_END: IsrBasicStackFrame.r2 -->

<!-- ANCHOR: IsrBasicStackFrame.r3 -->
The value of register `r3` pushed onto the stack at the time of context saving.
<!-- ANCHOR_END: IsrBasicStackFrame.r3 -->

<!-- ANCHOR: IsrBasicStackFrame.r12 -->
The value of register `r12` pushed onto the stack at the time of context saving.
<!-- ANCHOR_END: IsrBasicStackFrame.r12 -->

<!-- ANCHOR: IsrBasicStackFrame.r14_lr -->
The value of register `r14` (alias `LR`, the _Link Register_) pushed onto the stack at the time of context saving.
<!-- ANCHOR_END: IsrBasicStackFrame.r14_lr -->

<!-- ANCHOR: IsrBasicStackFrame.return_address -->
The value of the the return address.

This is the address loaded into `PC` when the ISR returns.  This depends on the type of exception, see Section B1.5.6 of the ARMv7-M Architecture Reference Manual for details including the `ReturnAddress()` pseudo-code.
<!-- ANCHOR_END: IsrBasicStackFrame.return_address -->

<!-- ANCHOR: IsrBasicStackFrame.xpsr -->
Stacked bits from the various Program Status Registers.

See Section B1.5.6 of the ARMv7-M Architecture Reference Manual for details.
<!-- ANCHOR_END: IsrBasicStackFrame.xpsr -->

<!-- ANCHOR: HasIsrBasicStackFrameMut -->
Unsafe trait providing a *mutable* Basic ISR Stack Frame as per Section B1.5.7 of the ARMv7-M Architecture Reference Manual.

_The use-cases for this trait are very limited - did you perhaps mean to use the *immutable* [`HasIsrBasicStackFrame`], instead ?_

Highly unsafe !  Any modification to the pushed registers will result in _Undefined Behaviour_ according to the rules of
[Inline Assembly](https://doc.rust-lang.org/reference/inline-assembly.html#r-asm.rules.reg-not-output) !

An example use-case of when mutating the stacked registers does not result in _Undefined Behaviour_ is that of Syscalls.  In this
scenario, the `SV_CALL` interrupt is invoked synchronously in respect to the main thread via an assembly-language stub.  This means any
mutations are able to be codified as `out` arguments in the stub in order not interfere with Rust's assumptions and optimisations.  Any
ISR that can be _invoked asynchronously_ to program flow should _not use this trait_ !
<!-- ANCHOR_END: HasIsrBasicStackFrameMut -->

<!-- ANCHOR: HasIsrBasicStackFrameMut.basic_stack_frame_mut -->
A *mutable* Basic ISR Stack Frame as per Section B1.5.7 of the ARMv7-M Architecture Reference Manual.

_The use-cases for this function are very limited - did you perhaps mean to use the *immutable* [`HasIsrBasicStackFrame::basic_stack_frame`], instead ?_

Highly unsafe !  Any modification to the pushed registers will result in _Undefined Behaviour_ according to the rules of
[Inline Assembly](https://doc.rust-lang.org/reference/inline-assembly.html#r-asm.rules.reg-not-output) !

An example use-case of when mutating the stacked registers does not result in _Undefined Behaviour_ is that of Syscalls.  In this
scenario, the `SV_CALL` interrupt is invoked synchronously in respect to the main thread via an assembly-language stub.  This means any
mutations are able to be codified as `out` arguments in the stub in order not interfere with Rust's assumptions and optimisations.  Any
ISR that can be _invoked asynchronously_ to program flow should _not use this function_ !
<!-- ANCHOR_END: HasIsrBasicStackFrameMut.basic_stack_frame_mut -->

<!-- ANCHOR: HasIsrBasicStackFrame -->
Unsafe trait providing a Basic ISR Stack Frame as per Section B1.5.7 of the ARMv7-M Architecture Reference Manual.

Highly unsafe !  It's hard to know where to begin with all of the things that can go wrong here.  The basic assumption is that this trait is
only used during trampolining (see [`isr_fn_trampolines!`]) where the _stack pointer_ `SP` _should_ be valid on entry into the ISR, ie. not
`null`, correctly aligned, no over- / under-flow, no intervening `push` or `pop`, etc.  And of course, the assumption that the stack contents have
been properly and coherently stored, fenced and remain immutable for the duration of the ISR.  Also thrown into the mix is also the issue of
pointer / reference lifetime and provenance, which are encapsulated inside the trampoline.

In short, use only as a consumer when writing a target function for an ISR trampoline.
<!-- ANCHOR_END: HasIsrBasicStackFrame -->

<!-- ANCHOR: HasIsrBasicStackFrame.basic_stack_frame -->
Unsafe function providing a Basic ISR Stack Frame as per Section B1.5.7 of the ARMv7-M Architecture Reference Manual.

See the notes in the [`HasIsrBasicStackFrame`] trait for why this is `unsafe`.
<!-- ANCHOR_END: HasIsrBasicStackFrame.basic_stack_frame -->
