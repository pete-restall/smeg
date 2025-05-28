<!-- ANCHOR: module -->
Bootstrapping for the Rust runtime.

Most constructs in this module are hideously `unsafe` - but hopefully not unsound - due to initialising 'things' that Rust itself considers invariant, such as immutable `static`s.

In particular, implementations must avoid [_Undefined Behaviour_][UB] as documented by [The Rust Reference](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) and elaborated via [Learn Unsafe Rust](https://google.github.io/learn_unsafe_rust/undefined_behavior.html).

[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: module -->

<!-- ANCHOR: initialise -->
Initialise the Rust runtime, such as filling `.bss` and loading `.data` sections.

This function must be called _only once_ as the first action of the generic [`__smeg_os_entrypoint`][__smeg_os_entrypoint] entrypoint.

In a multi-core system there is potential for data races which in turn give rise to immediate [_Undefined Behaviour_][UB].  To prevent this, it is the responsibility of the microcontroller-specific entrypoint (eg. the assembly stub that sets the initial stack pointer before jumping to [`__smeg_os_entrypoint`][__smeg_os_entrypoint]) to ensure all cores other than `core 0` are held in reset, asleep or otherwise occupied by some form of 'busy loop' until they have been signalled that the OS is ready.  On signalling, those secondary cores become active and will call [`__smeg_os_entrypoint`][__smeg_os_entrypoint].

[__smeg_os_entrypoint]: ../../../smeg_os/fn.__smeg_os_entrypoint.html
[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: initialise -->
