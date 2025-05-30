<!-- ANCHOR: module -->
Test doubles for [`HasMcuCoreId`].
<!-- ANCHOR_END: module -->

<!-- ANCHOR: Dummy.core_id -->
Dummy implementation of [`HasMcuCoreId::core_id`].

Returns [`usize::default()`].
<!-- ANCHOR_END: Dummy.core_id -->

<!-- ANCHOR: StubForConstantMcuCoreId -->
Stub implementation of [`HasMcuCoreId`] for a given MCU Core ID.

The MCU's Core ID is stubbed to be the value of the `const` generic argument `MCU_CORE_ID`.
<!-- ANCHOR_END: StubForConstantMcuCoreId -->

<!-- ANCHOR: StubForConstantMcuCoreId.core_id -->
Stub implementation of [`HasMcuCoreId::core_id`].

Returns the stubbed constant from the `const` generic argument `MCU_CORE_ID`.
<!-- ANCHOR_END: StubForConstantMcuCoreId.core_id -->
