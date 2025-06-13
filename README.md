# smeg
## What is this ?
**s**meg is a **m**icrocontroller **e**nvironment and **g**lue

## Why ?
I enjoyed catching up and delving into C++ 23, but frankly I wasn't finding it a particularly productive language.  That was for quite a few reasons, but for example, when you need to know the difference between `decltype(x)` and `decltype((x))`, or the several different interpretations of `auto &&`, reading and writing code starts to become a bit bogged down and fraught with error - cognitive load and context retention that could be better spent on solving the problem at hand.  Writing tests to verify that code was doing _exactly_ what I needed it to was laborious, not to mention testing and boilerplate for all the permutations of `cv` qualifications, copy, move and assignment semantics, working around lack of reflection ('loophole' !)s, etc. etc.  There's just a lot.

I can see I've put off learning Rust for too long, so here we go.  I have to say that so far it's looking pretty awesome; it's immutable by default (yay !) and catches a lot of the bad stuff at compile-time that I tend to try and codify as tests (double yay !)  The build system and libraries are first-class citizens, although I'm not keen on the modules (they seem to make the dependency arrows point the wrong way, going up the directory tree...) and spaces for indentation is just...urrgh.

That said, first-class tests, features, proper macros, conditional compilation, an explicit error-handling mechanism, traits, matching, etc. etc. are very much to like.  Not to mention being forced to explicitly confront lifetime management, plus all the compile-time optimisations and elisions to keep things rather svelte in the final binary.  There are lots of modern goodies and enforced good practice in the language, although at the bare metal level things can still get a bit undefined a bit quickly, but such hairiness is to be expected.

So yeah, it's time to have a proper play and start writing an experimental OS as one of my longer-term projects.

## Some General Notes / Aide Mémoires
Target naming convention - `<manufacturer>-<board>-<variant>`, eg. `st-nucleo_l432kc-my_widget`.

Microcontroller naming convention - `<manufacturer>-<microcontroller>`, eg. `st-stm32l432kc`.

Requires nightly (currently unstable) because of `feature(linkage, naked_functions)` and also to build the cross-toolchain for custom targets, which in turn requires the rust source:
```
$ rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

Requires `cargo-binutils` to enable the `build.sh` script to produce raw binaries without knowing anything about toolchains:
```
$ cargo +nightly install cargo-binutils
$ rustup component add --toolchain nightly llvm-tools
```

Requires `cargo-llvm-cov` to enable code coverage:
```
$ cargo +nightly install cargo-llvm-cov
```

The `dev` profile (`debug` target) includes various debug assertions with panic strings, which increases the size of the `.data` section considerably.  The `release` profile (`release` target) does not include these, so there is no impact on `.data`.

## Crate Hierarchy
The core OS functionality is in the following crates:
* `smeg-os` - the top-level crate that has features to toggle the board-specific dependencies, as well as re-exports of the dependent userspace APIs for the application to consume.  This crate produces the binary and can be considered the composition root because it exposes a well-known endpoint as a callback from the MCU bootstrapper.  Since this crate knows about all of the top-level configured features it can augment the MCU-specific data supplied to the callback with higher-level data and then call into the board-specific entrypoint.
* `smeg-config` - the crate and build scripts that pull all of the `smeg_config.toml` files together (respecting component hierarchy) and construct a merged and monolithic `const` for type-safe use by all of the other crates.  Depends on nothing but the discovered `smeg_config.toml` files; note that whilst amendments to the discovered `smeg_config.toml` files should result in an automatic rebuild of the crate, the addition of a new `smeg_config.toml` will require a manual rebuild of the crate.  Any rebuild of this crate should automatically cascade builds of all dependents.
* `smeg-board-<manufacturer>-<board>-<variant>` - the board-specific crate included by `smeg-os`, which in turn includes crates to handle the board- and hardware-specifics, such as which microcontroller is soldered onto the PCB that will be running the OS.  The `<variant>` element of the triple is intended to provide for scenarios where, for example, a family of boards offer largely the same functionality but differ in which MCU is mounted, or which peripherals are stuffed / active.  In that case it may make sense to have a single crate and conditionally include the appropriate MCU-specific crate.  Depends on other crates, such as `kernel`, `drivers`, etc.
* `smeg-mcu-<manufacturer>-<mcu>` - the entrypoint / bootstrapping code, including the Rust target JSON and linker scripts, one for the OS and one for the application.  Calls an `extern` symbol with MCU-specific details, such as linker symbols of interest, which is exported and implemented by a `smeg-os` function.  The `kernel` runtime initialisation function must have been called before the entrypoint returns.  Depends on other crates, such as `kernel`, `drivers`, etc.
* `smeg-kernel` - library containing kernel primitives, traits, generic entrypoint and runtime initialisation (eg. BSS, initialisation, .init, etc.), various abstractions.  Depends on nothing else.
* `smeg-driver-*` - library containing a driver.  Depends on `kernel`.

Other infrastructure / utility crates that are not directly part of the OS are:
* `smeg-build-utils` - utilities that can be used from build scripts, host-based integration tests, procmacros and the like.
* `smeg-testing-host-utils` - host-based (as opposed to device-based) integration testing utilities, boilerplate and support.

TODO: how to generate a separate linker script for the application with all of the kernel's linked symbols in it ?  Or should we even bother ?

## General Notes
* Tasks that exit (eg. one-shot) should be able to share a pool of stacks, ie. run as overlays.  This introduces the possibility of deadlocking, however - if another task from the same pool needs to be invoked to service a request from an already running task in the same pool, but there is no available stack, then there is deadlock.
* An ISR should be able to create / switch to a pre-emptive task without going through the scheduler, in order to offload its work to a non-ISR context but without incurring extra context-switching and scheduling overhead.  This means each MCU core needs to have its own list of (prioritised) tasks.  Each core needs its own scheduler anyway, so the idea is that a (user-space) scheduler task can run on each core and pull tasks from a shared list of tasks that are able to run on any core, and then assign them to the core-specific priority queues.  This should allow each core to control its own workload, pulling tasks from / pushing tasks back to the shared 'runnable' queues as necessary.  It also allows an easy way to implement core affinity and locality for tasks that absolutely must, or might otherwise benefit from, running on the same core.  Running a scheduler on each core means that it can acquire locks on the shared queues but use a Syscall that is guaranteed to invoke the ISR on the same core, thus avoiding locks / critical sections on the core-specific queue manipulation (at least in the case of ARM with `SV_CALL` and `PENDSV`).

## Important Points to Remember (GOTCHAS !)
### Rust and Memory-Mapped I/O (MMIO)
* Rust references are `dereferenceable` in LLVM parlance, which means they can be read speculatively.  Do _NOT_ have a reference to any MMIO or register or any non-memory 'entity' for this reason.  Side-effects matter.  This rule also excludes `VolatileCell`, `UnsafeCell`, etc. which internally manipulate references.  Use `core::ptr::read_volatile` and `core::ptr::write_volatile`, but note that it is **Undefined Behaviour** if two threads both try (any combination of) volatile read or write to the same location at the same time; `volatile != atomic`.
* Because Rust references are `dereferenceable`, do not create a pointer to MMIO _from a reference_; instead, use the raw pointer operators `&raw const ...` and `&raw mut ...`.  Do not create references to _ANY_ MMIO structure (eg. a register bank); it's all got to be done through pointers to avoid straying into UB.
* A `compiler_fence` is required if the order of volatile operations needs to be maintained relative to any non-volatile operations in the block.
* Not MMIO, but stacks are within Rust's memory model.  Mutating stack frames via pointers from a Syscall ISR (eg. for returning values in registers, as is typical with Syscall interfaces) is technically UB but working around this to provide another interface is more complex and less efficient, especially when a multi-core architecture would complicate the use of mutable `static`s.  Since this is confined to Syscalls (ie. synchronous with program execution) and triggered from assembly language stubs, we can control the visibility of the side-effects in the desired manner.  Technically UB but actually well defined for the given architecture.  There are more valid issues to be raised regarding provenance of stack frame pointer / reference being marshalled across the Rust / assembly / ISR boundary as a `usize`...

### Linker Script
* LLD is _NOT_ LD.  It differs in some subtle ways and will not produce the same output.  Documentation is poor-to-non-existent.
* Without the `PERIPHERALS` memory region or if placing the `.peripherals.*` sections somewhere not immediately after the `FLASH` / `SRAM` regions, weird stuff happens.  The program links, but the addresses are in weird places, even though their absolute addresses were specified as part of the `SECTION <addr> : { ... }` definition.  It's like they're treated as orphan sections, even though they're declared as `SHT_NOBITS`.
* Crates are built as dynamic shared objects (PIC), not static shared objects.
* Link-Time-Optimisation (LTO) and garbage collection mean that even `#[used]` items (and other techniques such as `EXTERN(<symbol>)` in the linker script) do not work if the linker _thinks_ there is no path to the symbols.  Therefore, the initial commit has a bunch of dummy public functions that ensure each crate calls into its dependencies.  Hopefully this will go away once code is added that actually uses the dependencies, but it may be an idea to keep these dummy functions (or something similar) just to enforce the inter-crate linkage and prevent subtle gotchas with code not being included in the final binary in future.

### Bootstrapping
The booting of a microcontroller is obviously an implementation detail specific to the microcontroller itself, thus belongs with the other microcontroller-specific modules in its respective `smeg-mcu-<manufacturer>-<mcu>` crate.  The problem is that the Composition Root is the `smeg-os` binary, because this configures the OS as a whole and depends on all the lower-level crates, including `smeg-board-<name>-<variant>` and `smeg-mcu-<manufacturer>-<mcu>`.  There is no way to get information from `smeg-os` into the bootstrapper, so there are two solutions - we either expose a well-known endpoint (in `smeg-os`) that each bootstrapper can call, or we create another crate that is a binary target with just the bootstrapper in it, that calls into `smeg-os`; the implications of the latter are that `smeg-os` is no longer the composition root and there will be `N` binary targets with largely the same boilerplate in them, one for each microcontroller.  I have chosen the former approach, a well-known endpoint.

The well-known endpoint that the bootstrapper calls must be an `extern` because `smeg-os` depends on `smeg-mcu-*`, which cannot (nor would we want it to) depend circularly on `smeg-os`.  Unfortunately, Rust does not allow generics across `extern` declarations (not even the `Rust` ABI), which means the MCU implementation cannot parameterise the entrypoint with information about itself (such as linker sections of interest, number of cores, etc.)  As a result, the `__smeg_os_entrypoint` is `extern "C"` and paramterless.

Since `smeg-os` uses features to include and configure board- and driver-specific crates, we simply expect each (conditionally compiled / included) board crate to expose some well-known types in a `bootstrapping` module that implement traits declared in the kernel; the board always knows what MCU is has soldered to it, so it can re-export the relevant types before calling into the kernel to initialise itself, with the injected dependencies.

The rather circuitous bootstrapping flow that facilitates the desired Dependency Inversion is thus `mcu` (power-on / reset) -> `os` (unparameterised jump) -> `kernel` (composed / parameterised).

Another important note is that the bootstrapping process is *very* `unsafe` - the Rust runtime will not have been initialised before the jump into `smeg-os`.  Technically, running any Rust code before any `static` is initialised (whether read or not) is *Undefined Behaviour*.  Since this is an operating system, the only way around that is to write a bunch of stuff to `memcpy` the `.data` sections and `memset` the `.bss` sections in another language like assembly (or C ??? :-D) before calling Rust, which will also need doing for each microcontroller.  Or, we just accept this is *technically Unsound* and ensure that the Rust that does run in the early bootstrapping process does absolutely nothing except call into the compiler intrinsics for `memcpy`, `memset` and some other `core::mem` / `core::ptr` stand-alone foundational primitives.  I chose the latter.  The one special case is for the `std` builds that simulate the system on the host - since they're built on `std` (and leveraging `crt0`, `libc`, etc.) we do not need to call `.init`s, zero `.bss`, copy `.data` or handle any other runtime chore.

### Despair
There is the notion of Despair, which is beyond Panic.  It is sometimes possible for the OS to recover from `panic!(...)`, but there is no way out of `despair!(...)`.  The default handler will loop forever with a `usize` on the top of the stack that indicates the error code from the `KernelErrorCode` enum and an ID of an error tag that can be looked up in the ELF `.smeg.tags.errors` section, which will reveal the source location and possibly something helpful diagnostic-wise.

Since despair cannot be recovered from, the only sensible _default_ course of action is the infinite loop, but it is also possible to override the `__smeg_is_in_despair` handler to do something more useful such as call for help, blink an LED, toggle a GPIO or UART, etc.  The caveat is that the handler must be a hand-rolled assembly language routine that is specific to the board since absolutely nothing can be assumed about the state of the device; we're probably in _Undefined Behaviour_ territory.  For example, it is possible that the reason for despair was due to a faulty linker script or other code generation issue, trashing memory and leaving an inky wake from which there is no escape.  So best to avoid those higher-level languages such as Rust and Esperanto.

## Potentially Useful Crates
* goblin - ELF (and other) binary parsing and loading
* serde - comprehensive serialisation, including `no_std` compatibility
