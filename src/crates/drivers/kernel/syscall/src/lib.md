<!-- ANCHOR: module -->
Driver providing an interface for System Calls (_Syscalls_).

System Calls are essentially software interrupts that can be used by unprivileged code to request services from the Kernel or Drivers (ie. privileged code).  The exact mechanism is a hardware- and architecture-specific implementation detail.
<!-- ANCHOR_END: module -->

<!-- ANCHOR: SyscallResult -->
The result of the Syscall execution, either `Ok(())` or `Err(KernelError)`.
<!-- ANCHOR_END: SyscallResult -->
