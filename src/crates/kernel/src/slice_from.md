<!-- ANCHOR: try_slice_from -->
Attempt to create a slice `[T]` from two `*const T` pointers, `[start, past_end)`.

Returns `None` if any of the sanity checks fail, such as passing unaligned pointers for the `T` type, or attempting to take a slice of Zero-Sized Types, otherwise the return value will be `Some(&'a [T])`.

The intended use-case for this function is to allow slices to be created from linker-supplied symbols, such as when building the Syscall vector tables at link-time, but there are other scenarios in which it may prove useful.

Note that this function is `unsafe` because, although a lot of sanity checks are present, it is impossible to prevent all _Undefined Behaviour (UB)_.  For example, this function cannot ensure that `start` and `past_end` are both within the same allocation; see [`from_raw_parts`][core::slice::from_raw_parts] for further details and _UB_ specifics.
<!-- ANCHOR_END: try_slice_from -->

<!-- ANCHOR: slice_from_unchecked -->
Create a slice `[T]` from two `*const T` pointers, `[start, past_end)`.

Very unsafe due to a lot of potential for _Undefined Behaviour (UB)_; see [`from_raw_parts`][core::slice::from_raw_parts] for further details and _UB_ specifics.  The only sanity check is ensuring that Zero-Sized Types are not passed, since that is a pure compile-time evaluation.
<!-- ANCHOR_END: slice_from_unchecked -->
