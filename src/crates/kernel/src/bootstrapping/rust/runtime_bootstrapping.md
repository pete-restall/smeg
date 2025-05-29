<!-- ANCHOR: RuntimeBootstrapping -->
Bootstrap the Rust runtime.

Runtime bootstrapping must be performed as the very first action after leaving the microcontroller's minimal stub of assembly that runs in response to power-on and reset events.  Many of Rust's invariants are violated until the bootstrapping has been completed, which places some very stringent constraints on this code and its dependencies.  In particular, the value of any `static` is not available and all RAM must be considered as _Uninitialised_.

The behaviours in this trait are not intended to be modified by implementors.  The expectation is that the trait is implemented for a Zero-Sized-Type (ZST) and the trait's associated types are defined to inject the desired behaviours.

Extreme care must be taken by the trait implementation to avoid [_Undefined Behaviour_][UB] because, rather obviously, the associated functions of this trait will be executed before there is a Rust runtime.  The alternative would be to write the runtime bootstrapping in, for example, the assembly stub for each microcontroller, before passing control to Rust, but this approach duplicates the functionality and unsafe code many times.

[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: RuntimeBootstrapping -->

<!-- ANCHOR: RuntimeBootstrapping.bootstrap -->
Bootstrap the Rust runtime, such as filling `.bss` and loading `.data` sections.

This function must be called _only once_ as the first action of the generic [`__smeg_os_entrypoint`][__smeg_os_entrypoint] entrypoint.

In a multi-core system there is potential for data races which in turn give rise to immediate [_Undefined Behaviour_][UB].  To prevent this, it is the responsibility of the microcontroller-specific entrypoint (eg. the assembly stub that sets the initial stack pointer before jumping to [`__smeg_os_entrypoint`][__smeg_os_entrypoint]) to ensure all cores other than `core 0` are held in reset, asleep or otherwise occupied by some form of 'busy loop' until they have been signalled that the OS is ready.  On signalling, those secondary cores become active and will call [`__smeg_os_entrypoint`][__smeg_os_entrypoint].

[__smeg_os_entrypoint]: ../../../smeg_os/fn.__smeg_os_entrypoint.html
[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: RuntimeBootstrapping.bootstrap -->
