<!-- ANCHOR: KernelError -->
Kernel error code and tag.

A convenience type providing a combination of [`KernelErrorCode`] and [`TaggedError`].  Able to fit within a `usize` whilst providing an error code with optional `u8` argument (the [`KernelErrorCode`]), as well as source location and descriptive text (the [`TaggedError`]).

See [`KernelErrorCode`] and [`TaggedError`] for further details.
<!-- ANCHOR_END: KernelError -->

<!-- ANCHOR: Result -->
Kernel specialisation for [`core::result::Result`].

The `Ok(...)` value can be of your choosing but the `Err(...)` value is always a [`KernelError`].
<!-- ANCHOR_END: Result -->

<!-- ANCHOR: ResultToUsizeResultConversion -->
Conversion methods for [`Result`] into [`UsizeResult`].

This trait is not intended to be implemented by anything other than [`Result<()>`].
<!-- ANCHOR_END: ResultToUsizeResultConversion -->

<!-- ANCHOR: ResultToUsizeResultConversion.as_usize_result -->
Moves a [`Result<()>`] into an equivalent [`UsizeResult`].

This is a safe operation but depends on internal implementation details of [`KernelErrorCode`] and [`TaggedError`] for efficiency.

This trait method is not intended to be implemented for anything other than [`Result<()>`].
<!-- ANCHOR_END: ResultToUsizeResultConversion.as_usize_result -->

<!-- ANCHOR: Result.as_usize_result -->
The only intended implementation of [`ResultToUsizeResultConversion::as_usize_result`].
<!-- ANCHOR_END: Result.as_usize_result -->

<!-- ANCHOR: UsizeKernelError -->
Opaque `struct` holding a `usize` representation of a [`Result<()>`].

This is an opaque `struct` that is `#[repr(transparent)]` for the purposes of layout optimisation and encapsulation.  This means that the `struct` has the same size and alignment requirements as the single `usize` primitive that it contains, which is done in order to take advantage of Rust's optimisation guarantees when used with [`UsizeResult`].

See [`UsizeResult`] for details.
<!-- ANCHOR_END: UsizeKernelError -->

<!-- ANCHOR: UsizeResult -->
Hold a machine-friendly `usize` representation of the more idiomatic [`Result`].

A [`UsizeResult`] is intended to fit in a single CPU register so that it can be moved easily and efficiently across boundaries, such as when passing results from architecture-specific Syscall interrupts back to their call sites.  This allows a bridge between the idiomatic Rust approach to Syscalls whilst remaining on friendly and efficient terms with the register-passing approach commonly taken by hardware.  Similar requirements exist for use-cases such as serialisation / deserialisation, message passing, error logging, etc.

See the following two documents for more information on Rust's requirements and guarantees on such layout optimisation:
- [https://doc.rust-lang.org/std/result/#representation](https://doc.rust-lang.org/std/result/#representation)
- [https://doc.rust-lang.org/std/option/index.html#representation](https://doc.rust-lang.org/std/option/index.html#representation)
<!-- ANCHOR_END: UsizeResult -->

<!-- ANCHOR: UsizeResultConversions -->
Unsafe trait allowing conversion of a `usize` into a [`UsizeResult`] as well as a [`UsizeResult`] back into a [`Result`].

This trait is not intended to be implemented by anything other than [`UsizeResult`].
<!-- ANCHOR_END: UsizeResultConversions -->

<!-- ANCHOR: UsizeResultConversions.from_usize_unchecked -->
Unsafe method to create a [`UsizeResult`] from a `usize` - _Undefined Behaviour_ if `usize` has no equivalent [`KernelError`] !

The only valid way to use this method is to pass it a `usize` value that originated from a [`UsizeKernelError`].  Any other value is considered _Undefined Behaviour_ even if the `usize` appears to be a correct representation.
<!-- ANCHOR_END: UsizeResultConversions.from_usize_unchecked -->

<!-- ANCHOR: UsizeResultConversions.as_result_unchecked -->
Unsafe method to convert a [`UsizeResult`] into a [`Result<()>`].

It is _Undefined Behaviour_ if the [`UsizeResult`] was not created from a `usize` that originated from a [`UsizeKernelError`].
<!-- ANCHOR_END: UsizeResultConversions.as_result_unchecked -->

<!-- ANCHOR: UsizeResult.from_usize_unchecked -->
Unsafe method to create a [`UsizeResult`] from a `usize` - _Undefined Behaviour_ if `usize` has no equivalent [`KernelError`] !

The only valid way to use this method is to pass it a `usize` value that originated from a [`UsizeKernelError`].  Any other value is considered _Undefined Behaviour_ even if the `usize` appears to be a correct representation.
<!-- ANCHOR_END: UsizeResult.from_usize_unchecked -->

<!-- ANCHOR: UsizeResult.as_result_unchecked -->
Unsafe method to convert a [`UsizeResult`] into a [`Result<()>`].

It is _Undefined Behaviour_ if the [`UsizeResult`] was not created from a `usize` that originated from a [`UsizeKernelError`].
<!-- ANCHOR_END: UsizeResult.as_result_unchecked -->
