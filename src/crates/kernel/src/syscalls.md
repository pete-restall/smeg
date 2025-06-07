<!-- ANCHOR: module -->
Basic and generic utilities for System Calls (_Syscalls_).

System Calls are essentially software interrupts that can be used to request services from the Kernel or Drivers (ie. privileged code) from unprivileged code.  The exact mechanism is a hardware- and architecture-specific implementation detail.
<!-- ANCHOR_END: module -->

<!-- ANCHOR: SyscallErrorCode -->
An error code returned when a Syscall does not complete properly.

Syscalls typically have a dedicated Task-specific buffer through which they can communicate, for example to pass arguments, return results and reference blocks of data for data transfer between the caller and callee.  Some Syscalls do not require large amount of data passing and a simple error code suffices.  Some Syscalls are invalid so there is no Task-specific buffer.  For the latter two scenarios, the `SyscallErrorCode` `enum` can convey basic meaning back to the caller about an error.

Syscalls are hardware-specific constructs and can typically only communicate using registers and / or the stack, so `SyscallErrorCode` needs to support a [`SyscallResult`] that has the same size as a `usize`, ie. a single machine word.
<!-- ANCHOR_END: SyscallErrorCode -->

<!-- ANCHOR: SyscallResult -->
The result of the Syscall execution, either `Ok(())` or `Err(SyscallErrorCode)`.

Syscalls are hardware-specific constructs and can typically only communicate using registers and / or the stack.  The [`SyscallResult`] thus needs to work in conjunction with [`SyscallErrorCode`] to support the lowest common denominator, which in practice means that it has the same size as a `usize`, ie. a single machine word.

Success is conveyed via a Zero-Sized Type (ZST) (ie. the unit `()`) and populating the Syscall- and Task-specififc buffer accordingly, if necessary.  The Syscall-specific documentation will have the details.
<!-- ANCHOR_END: SyscallResult -->
