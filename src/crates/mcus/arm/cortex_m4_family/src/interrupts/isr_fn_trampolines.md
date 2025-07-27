<!-- ANCHOR: isr_fn_trampolines -->
Macro to generate various types of trampoline for more convenient Interrupt Service Routines (ISRs).

The available formats are:
1. `fn TRAMPOLINE_FN() -> TARGET_FN<GENERICS>(TRAITS) -> "handler_main|thread_main|thread_process";`

Note the trailing `;`, which allows several trampolines to be defined within the same block.

## 1. Trampoline with a Reference to a Stack Frame
An example use of the macro might be:
```
# #[macro_use] extern crate smeg_mcu_arm_cortex_m4_family;
# use smeg_kernel::interrupts::IsrContext;
# trait Dependencies { type IsrContext: IsrContext; }
# unsafe fn on_syscall<D: Dependencies>(isr_context: &mut D::IsrContext) { }
isr_fn_trampolines! {
    fn sv_call_isr<Dependencies>() -> on_syscall() -> "thread_process";
}
```
The above constructs a trampoline function that can be inserted into the ISR Vector Table, which in turn calls your own ISR `on_syscall`, which does not require any generic types nor ISR context traits beyond those required for Cortex M4 compatibility.  The trampoline has the following signature:
```
# #[macro_use] extern crate smeg_mcu_arm_cortex_m4_family;
# use smeg_kernel::interrupts::IsrContext;
# trait Dependencies { type IsrContext: IsrContext; }
unsafe extern "C" fn sv_call_isr<D: Dependencies /* + ... */>() -> ! {
    // ...
# unimplemented!();
}
```
The target function in the above example is `on_syscall` and is for you to write.  It will be along the lines of:
```
# trait Dependencies { type IsrContext; }
unsafe fn on_syscall<D: Dependencies>(isr_context: &mut D::IsrContext) {
    // ...
}
```
Note the absence of the `!` (never) return - there is no epilogue or prologue to wire in as the trampoline takes care of it.  Once your `on_syscall` function returns, the trampoline returns from the ISR and directs the CPU to return control back to the main program using:
- `"handler_main"` - execution context stays in `Handler` mode and uses the `Main` stack (`MSP`)
- `"thread_main"` - switch execution context to `Thread` mode and use the `Main` stack (`MSP`)
- `"thread_process"` - switch execution context to `Thread` mode and use the `Process` stack (`PSP`)

Also note that additional trait bounds, marked `/* + ... */` above, can be requested by the ISR implementation and are supplied to the trampoline by the comma-separated `TRAITS` in the usage template at the start of this section.  Likewise, boundless generic arguments can be injected into the target function using a `<GENERICS>` template before `(TRAITS)`.  These `GENERICS` are able to use the implicitly defined `C` if they need a type for the [`IsrContext`].  This provides an easy way to inject (type) dependencies into ISRs - the trampolines will not generally be available to tests since they depend on architecture-specific details, but by providing generic parameters there is a seam for [Test Double](smeg_kernel::test_doubles) substitution.
<!-- ANCHOR_END: isr_fn_trampolines -->
