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

<!-- ANCHOR: SyscallResultUsizeConversion -->
Trait allowing conversion of a [`SyscallResult`] to and from a `usize`.

Unfortunately the functionality in this trait cannot be implemented using [`From`] since the underlying type of [`SyscallResult`] is [`Result`], which is not defined in this crate.  The unsafe conversion would also not lend itself to the standard [`From`] contract.

This trait is not intended to be implemented by anything other than [`SyscallResult`].
<!-- ANCHOR_END: SyscallResultUsizeConversion -->

<!-- ANCHOR: SyscallResultUsizeConversion.into_usize -->
Convert a [`SyscallResult`] to a `usize`.

A [`SyscallResult`] is intended to fit in a single CPU register so that it can be passed easily from the Syscall interrupt back to its caller, allowing a more idiomatic approach to Syscalls whilst remaining on friendly and efficient terms with the lowest common denominator of underlying hardware.
<!-- ANCHOR_END: SyscallResultUsizeConversion.into_usize -->

<!-- ANCHOR: SyscallResultUsizeConversion.from_usize_unchecked -->
Convert a `usize` into a [`SyscallResult`].

Highly unsafe and will happily cause [_Undefined Behaviour_][UB] if the given value is not a valid [`SyscallResult`] representation.  The *only* intended usage of this method is at a Syscall call-site, transmuting the `usize` stored by the interrupt handler in a register or on the stack back into a [`SyscallResult`].

If the `value` argument was not constructed using [`into_usize`] then it is considered [_Undefined Behaviour_][UB].

[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: SyscallResultUsizeConversion.from_usize_unchecked -->

<!-- ANCHOR: SyscallResult.into_usize -->
Convert a [`SyscallResult`] to a `usize`.

See [`SyscallResultUsizeConversion`] for the specifics.  This implementation is intended to be the *only* implementation of the trait method.
<!-- ANCHOR_END: SyscallResult.into_usize -->

<!-- ANCHOR: SyscallResult.from_usize_unchecked -->
Convert a `usize` into a [`SyscallResult`].

See [`SyscallResultUsizeConversion`] for the specifics.  This implementation is intended to be the *only* implementation of the trait method.
<!-- ANCHOR_END: SyscallResult.from_usize_unchecked -->
