<!-- ANCHOR: module -->
[Test Doubles](https://martinfowler.com/bliki/TestDouble.html) to aid testing of the [`super`] module.
<!-- ANCHOR_END: module -->

<!-- ANCHOR: Dummy -->
A common type for representing a Dummy.

The concept of a Dummy is universal, hence a single `struct`.  Context-specific traits can be defined for this `struct` to enable each testing scenario.
<!-- ANCHOR_END: Dummy -->

<!-- ANCHOR: Stub -->
Easily create simple stubs with [`From`].

Some `struct`s can be created with a default, potentially non-deterministic, set of values.  What that looks like depends on context, but using this `struct` in conjunction with the [`From`] trait allows a convenient way to get such a value.
<!-- ANCHOR_END: Stub -->
