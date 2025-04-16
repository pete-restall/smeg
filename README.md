# smeg
## What is this ?
**s**meg is a **m**icrocontroller **e**nvironment and **g**lue

## Why ?
I enjoyed catching up and delving into C++ 23, but frankly I wasn't finding it a particularly productive language.  That was for quite a few reasons, but for example, when you need to know the difference between `decltype(x)` and `decltype((x))`, or the several different interpretations of `auto &&`, reading and writing code starts to become a bit bogged down and fraught with error - cognitive load and context retention that could be better spent on solving the problem at hand.  Writing tests to verify that code was doing _exactly_ what I needed it to was laborious, not to mention testing and boilerplate for all the permutations of `cv` qualifications, copy, move and assignment semantics, working around lack of reflection ('loophole' !)s, etc. etc.  There's just a lot.

I can see I've put off learning Rust for too long, so here we go.  I have to say that so far it's looking pretty awesome; it's immutable by default (yay !) and catches a lot of the bad stuff at compile-time that I tend to try and codify as tests (double yay !)  The build system and libraries are first-class citizens, although I'm not keen on the modules (they seem to make the dependency arrows point the wrong way, going up the directory tree...) and spaces for indentation is just...urrgh.

That said, first-class tests, features, proper macros, conditional compilation, an explicit error-handling mechanism, traits, matching, etc. etc. are very much to like.  Not to mention being forced to explicitly confront lifetime management, plus all the compile-time optimisations and elisions to keep things rather svelte in the final binary.  There are lots of modern goodies and enforced good practice in the language, although at the bare metal level things can still get a bit undefined a bit quickly, but such hairiness is to be expected.

So yeah, it's time to have a proper play and start writing an experimental OS as one of my longer-term projects.

## Some General Notes / Aide Mémoires
Target naming convention - `<manufacturer>-<board>-<configuration>`, eg. `st-nucleo_l432kc-my_widget`.

Microcontroller naming convention - `<manufacturer>-<microcontroller>`, eg. `st-stm32l432kc`.

Requires nightly (currently unstable) to build the cross-toolchain for custom targets, which in turn requires the rust source:
```
$ rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

Requires `cargo-binutils` to enable the `build.sh` script to produce raw binaries without knowing anything about toolchains:
```
$ cargo +nightly install cargo-binutils
$ rustup component add --toolchain nightly llvm-tools
```

The `dev` profile (`debug` target) includes various debug assertions with panic strings, which increases the size of the `.data` section considerably.  The `release` profile (`release` target) does not include these, so there is no impact on `.data`.

## Crate Hierarchy
* `smeg-os` - the top-level crate that has features to toggle the board-specific dependencies, as well as re-exports of the dependent userspace APIs for the application to consume.  This crate produces the binary and can be considered the composition root because it exposes a well-known endpoint as a callback from the MCU bootstrapper.  Since this crate knows about all of the top-level configured features it can augment the MCU-specific data supplied to the callback with higher-level data and then call into the board-specific entrypoint.
* `smeg-board-<manufacturer>-<board>` - the board-specific crate included by `smeg-os`, which in turn includes crates to handle the board- and hardware-specifics, such as which microcontroller is soldered onto the PCB that will be running the OS.  The `<configuration>` element of the triple is intended to provide for scenarios where, for example, a family of boards offer largely the same functionality but differ in which MCU is mounted.  In that case it may make sense to have a single crate and conditionally include the appropriate MCU-specific crate.  Depends on other crates, such as `kernel`, `drivers`, etc.
* `smeg-mcu-<manufacturer>-<mcu>` - the entrypoint / bootstrapping code, including the Rust target JSON and linker scripts, one for the OS and one for the application.  Calls an `extern` symbol with MCU-specific details, such as linker symbols of interest, which is exported and implemented by a `smeg-os` function.  The `kernel` runtime initialisation function must have been called before the entrypoint returns.  Depends on other crates, such as `kernel`, `drivers`, etc.
* `smeg-kernel` - library containing kernel primitives, traits, generic entrypoint and runtime initialisation (eg. BSS, initialisation, .init, etc.), various abstractions.  Depends on nothing else.
* `smeg-driver-*` - library containing a driver.  Depends on `kernel`.

TODO: how to generate a separate linker script for the application with all of the kernel's linked symbols in it ?  Or should we even bother ?

## Important Points to Remember (GOTCHAS !)
### Rust and Memory-Mapped I/O (MMIO)
* Rust references are `dereferenceable` in LLVM parlance, which means they can be read speculatively.  Do _NOT_ have a reference to any MMIO or register or any non-memory 'entity' for this reason.  Side-effects matter.  This rule also excludes `VolatileCell`, `UnsafeCell`, etc. which internally manipulate references.  Use `core::ptr::read_volatile` and `core::ptr::write_volatile`, but note that it is **Undefined Behaviour** if two threads both try (any combination of) volatile read or write to the same location at the same time; `volatile != atomic`.
* Because Rust references are `dereferenceable`, do not create a pointer to MMIO _from a reference_; instead, use the raw pointer operators `&raw const ...` and `&raw mut ...`.  Do not create references to _ANY_ MMIO structure (eg. a register bank); it's all got to be done through pointers to avoid straying into UB.
* A `compiler_fence` is required if the order of volatile operations needs to be maintained relative to any non-volatile operations in the block.

### Linker Script
* LLD is _NOT_ LD.  It differs in some subtle ways and will not produce the same output.  Documentation is poor-to-non-existent.
* Without the `PERIPHERALS` memory region or if placing the `.peripherals.*` sections somewhere not immediately after the `FLASH` / `SRAM` regions, weird stuff happens.  The program links, but the addresses are in weird places, even though their absolute addresses were specified as part of the `SECTION <addr> : { ... }` definition.  It's like they're treated as orphan sections, even though they're declared as `SHT_NOBITS`.
* Crates are built as dynamic shared objects (PIC), not static shared objects.
* Link-Time-Optimisation (LTO) and garbage collection mean that even `#[used]` items (and other techniques such as `EXTERN(<symbol>)` in the linker script) do not work if the linker _thinks_ there is no path to the symbols.  Therefore, the initial commit has a bunch of dummy public functions that ensure each crate calls into its dependencies.  Hopefully this will go away once code is added that actually uses the dependencies, but it may be an idea to keep these dummy functions (or something similar) just to enforce the inter-crate linkage and prevent subtle gotchas with code not being included in the final binary in future.
