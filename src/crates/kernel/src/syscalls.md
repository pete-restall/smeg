<!-- ANCHOR: module -->
Basic and generic utilities for System Calls (_Syscalls_).

System Calls are essentially software interrupts that can be used for unprivileged code to request services from the Kernel or Drivers (ie. privileged code).  The exact mechanism is a hardware- and architecture-specific implementation detail.
<!-- ANCHOR_END: module -->

<!-- ANCHOR: McuSyscallInvocation -->
Trait providing a portable mechanism to invoke a Syscall.

The number of Syscalls provided varies from machine to machine, with some encoding the value inside an opcode.  A `u8` is thus a reasonable lowest-common-denominator assumption.  In practice it is assumed that there will be _way_ less than 256 Syscalls, since these represent the lowest-level services provided by the OS to an application - most Syscalls are encapsulated in and utilised by more useful and generic libraries.
<!-- ANCHOR_END: McuSyscallInvocation -->

<!-- ANCHOR: McuSyscallInvocation.invoke_syscall -->
Emit the machine-specific instructions necessary to invoke a Syscall.

The exact mechanism is undefined and varies from machine to machine, but this trait commits to a contract of taking a `u8` to identify the Syscall and returning a [`SyscallResult`] to inform the caller whether the Syscall was successful or not.

If Syscalls require more data then then they will typically have a dedicated Task-specific buffer through which data can be populated, both by the caller for input and by the Syscall for output.  For example, to pass arguments, return detailed results or to reference blocks of data for transfers between the caller and callee.

Syscalls are hardware-specific constructs and can typically only communicate using registers and / or the stack, meaning `SyscallResult` will need to be marshalled across the Rust-interrupt boundary using a `usize`.  See [`UsizeResult`][smeg_kernel::errors::UsizeResult] which has been provided for this purpose.
<!-- ANCHOR_END: McuSyscallInvocation.invoke_syscall -->

<!-- ANCHOR: SyscallResult -->
The result of the Syscall execution, either `Ok(())` or `Err(KernelError)`.
<!-- ANCHOR_END: SyscallResult -->
