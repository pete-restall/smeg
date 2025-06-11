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
