<!-- ANCHOR: module -->
<!-- ANCHOR_END: module -->

<!-- ANCHOR: DataSectionInitialiser -->
Initialise a _data_ linker section, eg. `.data`.

A hideously `unsafe` bootstrapping trait, responsible for loading linker-defined data sections such as `.data` with their initial values.

Extreme care must be taken by implementators to avoid [_Undefined Behaviour_][UB].  This is because the associated functions of this trait will be executed before there is a Rust runtime, meaning many behaviours that Rust considers invariant may not necessarily hold during that execution.  In particular, the value of any `static` is not available and all RAM must be considered as _Uninitialised_ - see [`MaybeUninit<T>`][MaybeUninit].

An incorrect linker script can give rise to the possibility of wiping arbitrary areas of system memory, allocating insufficient `.data` to cover the values in ROM or conversely allocating too much `.data` so that ROM is read past the end of the table and into other areas, possibly executable code or even non-ROM address spaces.  These issues are unavoidable regardless of whether the functionality provided by this trait is implemented in Rust or assembly before any Rust is invoked; at least by having runtime bootstrapping in Rust we can have a single portable implementation to reduce code duplication and surface area for [_Undefined Behaviour_][UB].

[MaybeUninit]: https://doc.rust-lang.org/core/mem/union.MaybeUninit.html
[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: DataSectionInitialiser -->

<!-- ANCHOR: DataSectionInitialiser.load_data_section -->
Load a seeded table of pre-computed data from a ROM table into RAM.

Extreme care must be taken by implementators to avoid [_Undefined Behaviour_][UB].  Implementations of this function will be called before there is a Rust runtime, meaning many behaviours that Rust considers invariant may not necessarily hold during execution.  In particular, the value of any `static` is not available and all RAM must be considered as _Uninitialised_ - see [`MaybeUninit<T>`][MaybeUninit].  It is the job of this function to initialise those `static` variables but there may be any number of `.data` sections and implementors cannot rely on any particular order of calls.

The address of the first byte in the RAM block to be filled is `ram_start` and the _next byte after the end of the block_ is `ram_past_end`.  The start of the data to load is given by `rom_start`; there is no need to mark the end of the ROM table since the RAM block will determine its size.  These arguments will be passed from symbols defined by the linker script.

The linker script must ensure `ram_start`, `ram_past_end` and `rom_start` are all `usize`-aligned and sized for the target architecture.

The linker script must ensure that the number of bytes in RAM matches the number of bytes in ROM.

The linker script must ensure that `ram_start <= ram_past_end` when it comes to their addresses in memory.

The linker script is also responsible for ensuring the RAM sections are indeed in RAM and do not cover memory areas with non-RAM read/write semantics, such as registers and MMIO.

[MaybeUninit]: https://doc.rust-lang.org/core/mem/union.MaybeUninit.html
[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: DataSectionInitialiser.load_data_section -->

<!-- ANCHOR: DataSectionInitialiserWithChecks -->
Initialise a _data_ linker section, eg. `.data`.

An implementation of the [`DataSectionInitialiser`] trait with some sanity checks to [`despair!`] if there is detectable error.
<!-- ANCHOR_END: DataSectionInitialiserWithChecks -->

<!-- ANCHOR: DataSectionInitialiserWithChecks.load_data_section -->
Load a seeded table of pre-computed data from a ROM table into RAM.

See [`DataSectionInitialiser::load_data_section`] for implementation notes and assumptions.

It is possible to detect when `ram_start > ram_past_end` for address calculation, in which case the function will [`despair!`].

Note that it is still possible to fall into [_Undefined Behaviour_][UB] for scenarios that are not able to be detected by this function if the passed arguments are incorrect.  A few (non-exhaustive) examples being:
* if `ram_start`, `ram_past_end` and `rom_start` are not properly aligned (this could be detected and [`despair!`] in future)
* if ROM and RAM addresses overlap (this could be detected and [`despair!`] in future)
* if the block does not cover all memory that needs loading
* if the block extends over memory not supposed to be in the section
* if the contents of the ROM table initialise a variable with an unrepresentable value

[UB]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
<!-- ANCHOR_END: DataSectionInitialiserWithChecks.load_data_section -->

<!-- ANCHOR: DataSectionInitialiserWithoutChecks -->
Initialise a _data_ linker section, eg. `.data`.

A minimal implementation of the [`DataSectionInitialiser`] trait that assumes its input arguments are correct and makes no attempt at checking.

See [`DataSectionInitialiser::load_data_section`] for implementation notes and assumptions.
<!-- ANCHOR_END: DataSectionInitialiserWithoutChecks -->

<!-- ANCHOR: DataSectionInitialiserWithoutChecks.load_data_section -->
Load a seeded table of pre-computed data from a ROM table into RAM.

A minimal implementation of the [`DataSectionInitialiser::load_data_section`] function that assumes its input arguments are correct and makes no attempt at checking.

See [`DataSectionInitialiser::load_data_section`] for implementation notes and assumptions.
<!-- ANCHOR_END: DataSectionInitialiserWithoutChecks.load_data_section -->
