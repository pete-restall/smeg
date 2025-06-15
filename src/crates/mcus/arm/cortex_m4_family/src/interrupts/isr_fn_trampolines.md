<!-- ANCHOR: isr_fn_trampolines -->
Macro to generate various types of trampoline for more convenient Interrupt Service Routines (ISRs).

The available formats are:
1. `fn TRAMPOLINE_FN() -> TARGET_FN<TRAITS>() -> "handler_main|thread_main|thread_process";`

Note the trailing `;`, which allows several trampolines to be defined within the same block.

## 1. Trampoline with a Reference to a Stack Frame
An example use of the macro might be:
```
# #[macro_use] extern crate smeg_mcu_arm_cortex_m4_family;
# use smeg_mcu_arm_cortex_m4_family::interrupts::IsrBasicStackFrame;
isr_fn_trampolines! {
    fn sv_call_isr() -> on_syscall<>() -> "thread_process";
}
```
The above constructs a trampoline function that can be inserted into the ISR Vector Table, which in turn calls your own ISR `on_syscall`, which does not require any ISR context traits beyond those required for Cortex M4 compatibility.  The trampoline has the following signature:
```
unsafe extern "C" {
    unsafe fn sv_call_isr() -> !;
}
```
The target function in the above example is `on_syscall` and is for you to write.  It will be along the lines of:
```
# use core::borrow::BorrowMut;
# use core::convert::From;
# use smeg_kernel::interrupts::IsrContext;
# use smeg_mcu_arm_cortex_m4_family::interrupts::IsrContextImpl;
unsafe fn on_syscall<C: IsrContext + From<IsrContextImpl> + BorrowMut<IsrContextImpl> /* + ... */>(isr_context: &mut C) {
    // ...
}
```
Note the absence of the `!` (never) return - there is no epilogue or prologue to wire in as the trampoline takes care of it.  Once your `on_syscall` function returns, the trampoline returns from the ISR and directs the CPU to return control back to the main program using:
- `"handler_main"` - execution context stays in `Handler` mode and uses the `Main` stack (`MSP`)
- `"thread_main"` - switch execution context to `Thread` mode and use the `Main` stack (`MSP`)
- `"thread_process"` - switch execution context to `Thread` mode and use the `Process` stack (`PSP`)

Also note that additional traits, marked `/* + ... */` above, can be requested by the ISR implementation and are supplied to the trampoline by the comma-separated `TRAITS` in the usage template at the start of this section.
<!-- ANCHOR_END: isr_fn_trampolines -->
