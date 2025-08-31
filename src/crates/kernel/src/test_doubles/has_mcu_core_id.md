<!-- ANCHOR: module -->
Test doubles for [`HasMcuCoreId`].
<!-- ANCHOR_END: module -->

<!-- ANCHOR: Dummy.NumberOfMcuCores -->
Dummy value of [`HasMcuCoreId.NumberOfMcuCores`].

Alias for [`ConstUsize<1>`].
<!-- ANCHOR_END: Dummy.NumberOfMcuCores -->

<!-- ANCHOR: Dummy.mcu_core_id -->
Dummy implementation of [`HasMcuCoreId.mcu_core_id`].

Returns [`usize::default()`].
<!-- ANCHOR_END: Dummy.mcu_core_id -->

<!-- ANCHOR: StubForConstantMcuCoreId -->
Stub implementation of [`HasMcuCoreId`] for a given (constant) MCU Core ID.

The MCU's Core ID is stubbed to be the value of the `const` generic argument `MCU_CORE_ID`.
<!-- ANCHOR_END: StubForConstantMcuCoreId -->

<!-- ANCHOR: StubForConstantMcuCoreId.NumberOfMcuCores -->
Stub value of [`HasMcuCoreId.NumberOfMcuCores`].

The MCU's number of cores is stubbed to be the value of the `const` generic argument `NUMBER_OF_MCU_CORES`.
<!-- ANCHOR_END: StubForConstantMcuCoreId.NumberOfMcuCores -->

<!-- ANCHOR: StubForConstantMcuCoreId.mcu_core_id -->
Stub implementation of [`HasMcuCoreId.mcu_core_id`].

Returns the stubbed constant from the `const` generic argument `MCU_CORE_ID`.
<!-- ANCHOR_END: StubForConstantMcuCoreId.mcu_core_id -->

<!-- ANCHOR: StubHasMcuCoreId -->
Stub implementation of [`HasMcuCoreId`] for a given (runtime) MCU Core ID.
<!-- ANCHOR_END: StubHasMcuCoreId -->

<!-- ANCHOR: StubHasMcuCoreId.NumberOfMcuCores -->
Stub value of [`HasMcuCoreId.NumberOfMcuCores`].

The MCU's number of cores is stubbed to be the value of the `const` generic argument `NUMBER_OF_MCU_CORES`.
<!-- ANCHOR_END: StubHasMcuCoreId.NumberOfMcuCores -->

<!-- ANCHOR: StubHasMcuCoreId.mcu_core_id -->
Stub implementation of [`HasMcuCoreId.mcu_core_id`].

Returns the current stubbed Core ID.
<!-- ANCHOR_END: StubHasMcuCoreId.mcu_core_id -->

<!-- ANCHOR: StubHasMcuCoreId.with -->
Create a stub with the given Core ID.

Panics if the given value is out of range (`>= NUMBER_OF_MCU_CORES`).
<!-- ANCHOR_END: StubHasMcuCoreId.with -->

<!-- ANCHOR: StubHasMcuCoreId.with_unchecked -->
Create a stub with the given (possibly out-of-range) Core ID.

Does not panic if the given value is out of range (`>= NUMBER_OF_MCU_CORES`).
<!-- ANCHOR_END: StubHasMcuCoreId.with_unchecked-->
