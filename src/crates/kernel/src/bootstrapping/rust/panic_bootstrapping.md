<!-- ANCHOR: PanicBootstrapping -->
Bootstrap the Rust [`panic!`] infrastructure.

The behaviours in this trait are not intended to be modified by implementors.  The expectation is that the trait is implemented for a Zero-Sized-Type (ZST) and the trait's associated types are defined to inject the desired behaviours.

It is safe to [`panic!`] before this infrastructure has been bootstrapped, but the result will simply be [`despair!`](crate::despair).
<!-- ANCHOR_END: PanicBootstrapping -->

<!-- ANCHOR: PanicBootstrapping.bootstrap -->
Bootstrap the Rust [`panic!`] infrastructure.

Some panics can be, and indeed are _expected_ to be, recovered from.  For example, if an application task called [`panic!`] then it is expected that the operating system terminates it and clears up its resources; if should _not_ halt or otherwise disrupt the operating system or any other running tasks.  If the kernel calls [`panic!`] then there are some scenarios that could be recovered from, eg. by resetting the core (a 'soft-reboot') or the entire microcontroller, and some scenarios that cannot be recovered from.  The latter is simply a path to [`despair!`](crate::despair).
<!-- ANCHOR_END: PanicBootstrapping.bootstrap -->

<!-- ANCHOR: PanicBootstrapping.DefaultPanicBootstrapper -->
The default implementation of [`PanicBootstrapping`].

Currently the only implementation.  This may change in future.
<!-- ANCHOR_END: PanicBootstrapping.DefaultPanicBootstrapper -->
