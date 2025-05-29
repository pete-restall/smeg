<!-- ANCHOR: module -->
<!-- ANCHOR_END: module -->

<!-- ANCHOR: BssSectionInitialisation -->
Initialise a _Block Start Symbol_ (BSS) linker section, eg. `.bss`.

A hideously `unsafe` bootstrapping trait, responsible for filling linker-defined BSS sections such as `.bss` with a known value.

Extreme care must be taken by implementators to avoid [_Undefined Behaviour_][UB].  This is because the associated functions of this trait will be executed before there is a Rust runtime, meaning many behaviours that Rust considers invariant may not necessarily hold during that execution.  In particular, the value of any `static` is not available and all RAM must be considered as _Uninitialised_ - see [`MaybeUninit<T>`][MaybeUninit].

An incorrect linker script can give rise to the possibility of wiping arbitrary areas of system memory, or indeed not allocating sufficient `.bss` for the system's needs.  This is unavoidable regardless of whether the functionality provided by this trait is implemented in Rust or assembly before any Rust is invoked; at least by having runtime bootstrapping in Rust we can have a single portable implementation to reduce code duplication and surface area for [_Undefined Behaviour_][UB].

[MaybeUninit]: https://doc.rust-lang.org/core/mem/union.MaybeUninit.html
[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: BssSectionInitialisation -->

<!-- ANCHOR: BssSectionInitialisation.fill_bss_section -->
Fill a _Block Start Symbol_ (BSS) linker section with `fill_value`.

Extreme care must be taken by implementators to avoid [_Undefined Behaviour_][UB].  Implementations of this function will be called before there is a Rust runtime, meaning many behaviours that Rust considers invariant may not necessarily hold during execution.  In particular, the value of any `static` is not available and all RAM must be considered as _Uninitialised_ - see [`MaybeUninit<T>`][MaybeUninit].

The address of the first byte in the RAM block to be filled is `start` and the _next byte after the end of the block_ is `past_end`.  These arguments will be passed from symbols defined by the linker script.

The linker script must ensure both `start` and `past_end` are `usize`-aligned and sized for the target architecture, even though `fill_value` is a byte.

The linker script must ensure that `start <= past_end` when it comes to their addresses in memory.

The linker script is also responsible for ensuring the sections are in RAM and do not cover memory areas with non-RAM read/write semantics, such as registers and MMIO.

The `fill_value` is typically `0x00` for most sections where this function is used, such as `.bss`, but other sections may use different values.  For example, if initialising a stack section then it may be desirable to use an uncommon value to act as a 'low water mark' to aid size determination and optimisation.

[MaybeUninit]: https://doc.rust-lang.org/core/mem/union.MaybeUninit.html
[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: BssSectionInitialisation.fill_bss_section -->

<!-- ANCHOR: BssSectionInitialiserWithChecks -->
Initialise a _Block Start Symbol_ (BSS) linker section, eg. `.bss`.

An implementation of the [`BssSectionInitialisation`] trait with some sanity checks to [`despair!`] if there is detectable error.
<!-- ANCHOR_END: BssSectionInitialiserWithChecks -->

<!-- ANCHOR: BssSectionInitialiserWithChecks.fill_bss_section -->
Fill a _Block Start Symbol_ (BSS) linker section with `fill_value`.

See [`BssSectionInitialisation::fill_bss_section`] for implementation notes and assumptions.

It is possible to detect when `start > past_end` for address calculation, in which case the function will [`despair!`].

Note that it is still possible to fall into [_Undefined Behaviour_][UB] for scenarios that are not able to be detected by this function if the passed arguments are incorrect.  A few (non-exhaustive) examples being:
* if `start` and `past_end` are not properly aligned (this could be detected and [`despair!`] in future)
* if the block does not cover all memory that needs initialising
* if the block extends over memory not supposed to be in the section
* if `fill_value` initialises a variable with an unrepresentable value, such as `0xff` when the section contains a `bool` or an `enum`

[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: BssSectionInitialiserWithChecks.fill_bss_section -->

<!-- ANCHOR: BssSectionInitialiserWithoutChecks -->
Initialise a _Block Start Symbol_ (BSS) linker section, eg. `.bss`.

A minimal implementation of the [`BssSectionInitialisation`] trait that assumes its input arguments are correct and makes no attempt at checking.

See [`BssSectionInitialisation::fill_bss_section`] for implementation notes and assumptions.
<!-- ANCHOR_END: BssSectionInitialiserWithoutChecks -->

<!-- ANCHOR: BssSectionInitialiserWithoutChecks.fill_bss_section -->
Fill a _Block Start Symbol_ (BSS) linker section with `fill_value`.

A minimal implementation of the [`BssSectionInitialisation::fill_bss_section`] function that assumes its input arguments are correct and makes no attempt at checking.

See [`BssSectionInitialisation::fill_bss_section`] for implementation notes and assumptions.
<!-- ANCHOR_END: BssSectionInitialiserWithoutChecks.fill_bss_section -->
