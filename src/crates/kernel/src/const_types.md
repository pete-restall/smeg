<!-- ANCHOR: HasConstUsizeValue -->
Trait exposing a single `const VALUE: usize`.

The only reason for this trait to exist is because Rust does not allow `const` expressions that manipulate generic parameters.  See [`ConstUsize`] for more details.
<!-- ANCHOR_END: HasConstUsizeValue -->

<!-- ANCHOR: HasConstUsizeValue.VALUE -->
The value of the generic argument passed to the type's constant parameter `VALUE`.
<!-- ANCHOR_END: HasConstUsizeValue.VALUE -->

<!-- ANCHOR: ConstUsize -->
A Zero-Sized Type taking a single `usize` constant generic argument.

The only reason for this type to exist is because Rust does not (yet) allow `const` expressions that manipulate associated `const`s.  Using this rather hacky kludge, it is possible to have code like the following:
```
# use smeg_kernel::{ConstUsize, HasMcuCoreId};
/*
// Even if M::NUMBER_OF_CORES is a const, this (or variations on this theme) will not work
pub struct McuCoreLocal<M: HasMcuCoreId, T> {
    mcu: M,
    values: [T; M::NUMBER_OF_CORES]
}
*/

// Instead we can extract the generic argument with a bit of ugly bounds inference along the lines of:
pub struct McuCoreLocal<M, const N: usize, T> where M: HasMcuCoreId<NumberOfMcuCores = ConstUsize<N>>, [(); N]: Sized {
    mcu: M,
    values: [T; N]
}
```
<!-- ANCHOR_END: ConstUsize -->

<!-- ANCHOR: ConstUsize.VALUE -->
The value of the generic argument passed to the type's constant parameter `VALUE`.
<!-- ANCHOR_END: ConstUsize.VALUE -->
