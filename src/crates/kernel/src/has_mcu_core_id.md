<!-- ANCHOR: HasMcuCoreId -->
Trait to determine the current core's ID.

Provides an abstraction over the microcontroller- (MCU-) specific mechanism for determining the ID of the core currently executing the caller.

Core IDs must be unique integers in the range `[0, N)`, where `N` is the number of cores that are provided by he MCU hardware.  The core using ID `0` is treated as the _primary core_, which has significance in certain circumstances such as bootstrapping.  All other IDs represent _secondary_ cores.

Core IDs must not change without an intervening power cycle and the number of cores provided by an MCU is a compile-time `const` that cannot change.  A physical manifestation of a core does not need to be assigned the same ID after a power cycle but may be determined dynamically by hardware factors such as, for example, which core boots first.  It is up to the MCU entrypoint to synchronise such chaotic boots to prevent race conditions, and assign static IDs for the OS to use for the duration of power-up.

The mechanism for determining the ID may change depending on the context of the caller.  For example, if the ID is determined by reading from a CPU register then it may be that register is only readable from a privileged mode of execution, ie. in kernelspace.  As such, alternate mechanisms may need to be provided for bootstrapping, userspace, interrupt or indeterminate calling contexts.
<!-- ANCHOR_END: HasMcuCoreId -->

<!-- ANCHOR: HasMcuCoreId.mcu_core_id -->
Retrieves the ID of the core executing the caller.

The mechanism is unspecified and may involve something heavyweight such as a system call, or something lightweight such as a cached ID or reading from a register.  An appropriate implementation should be provided for the context of the caller.
<!-- ANCHOR_END: HasMcuCoreId.mcu_core_id -->

<!-- ANCHOR: McuSingleCore -->
Convenience implementation of [`HasMcuCoreId`] for the common degenerate case of a single-core architecture.
<!-- ANCHOR_END: McuSingleCore -->

<!-- ANCHOR: McuSingleCore.mcu_core_id -->
Returns a hard-coded `const` of `0`.
<!-- ANCHOR_END: McuSingleCore.mcu_core_id -->
