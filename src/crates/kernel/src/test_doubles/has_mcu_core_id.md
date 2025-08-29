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
Stub implementation of [`HasMcuCoreId`] for a given MCU Core ID.

The MCU's Core ID is stubbed to be the value of the `const` generic argument `MCU_CORE_ID`.
<!-- ANCHOR_END: StubForConstantMcuCoreId -->

<!-- ANCHOR: StubForConstantMcuCoreId.NumberOfMcuCores -->
Stub value of [`HasMcuCoreId.NumberOfMcuCores`].

Alias for the [`ConstUsize<{MCU_CORE_ID + 1}>`].
<!-- ANCHOR_END: StubForConstantMcuCoreId.NumberOfMcuCores -->

<!-- ANCHOR: StubForConstantMcuCoreId.mcu_core_id -->
Stub implementation of [`HasMcuCoreId.mcu_core_id`].

Returns the stubbed constant from the `const` generic argument `MCU_CORE_ID`.
<!-- ANCHOR_END: StubForConstantMcuCoreId.mcu_core_id -->
