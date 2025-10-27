use smeg_kernel::mem::*;

use crate::mem::*;

use super::SystemControlBlockMemoryAttributes;

// TODO: Some sort of accessor is required - the #[arm_register] attribute will define it - but the structure and implementation are yet to be decided
pub struct CpuidAccessor<'mem> {
    accessor: CellAccessor<'mem, ReadonlyCell<SystemControlBlockMemoryAttributes, Cpuid>>
}

#[mmio_register]
#[datasheet("DDI0403E.e", "B3.2.3", 598)]
#[ro(IMPLEMENTER,  0b11111111_0000_0000_000000000000_0000)]
#[ro(VARIANT,      0b00000000_1111_0000_000000000000_0000)]
#[ro(ARCHITECTURE, 0b00000000_0000_1111_000000000000_0000)]
#[ro(PARTNO,       0b00000000_0000_0000_111111111111_0000)]
#[ro(REVISION,     0b00000000_0000_0000_000000000000_1111)]
pub struct Cpuid(u32);
