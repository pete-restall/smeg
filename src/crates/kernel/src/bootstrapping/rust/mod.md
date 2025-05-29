<!-- ANCHOR: module -->
Bootstrapping for the Rust runtime.

Most constructs in this module are hideously `unsafe` - but hopefully not unsound - due to initialising 'things' that Rust itself considers invariant, such as immutable `static`s.

In particular, implementations must avoid [_Undefined Behaviour_][UB] as documented by [The Rust Reference](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) and elaborated via [Learn Unsafe Rust](https://google.github.io/learn_unsafe_rust/undefined_behavior.html).

[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: module -->
