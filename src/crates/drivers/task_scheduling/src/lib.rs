#![cfg_attr(not(any(test, feature = "std")), no_std)]

// TODO: An example of where a driver might live.  We will need MCU-specific driver crates, so the level of nesting ought to change.
// For example, drivers/dma/hal (crate smeg-driver-dma-hal) may provide traits that can be implemented by a dependent drivers/dma/st/stm32l432kc (crate smeg-driver-dma-st-stm32l432kc)
// Possibly.
