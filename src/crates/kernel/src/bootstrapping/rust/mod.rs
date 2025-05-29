#![doc = crate::docs::side_by_side_md!()]

mod bss_section_initialisation;
pub use bss_section_initialisation::*;

mod data_section_initialisation;
pub use data_section_initialisation::*;

mod mcu_memory_bootstrapping;
pub use mcu_memory_bootstrapping::*;

mod runtime_bootstrapping;
pub use runtime_bootstrapping::*;
