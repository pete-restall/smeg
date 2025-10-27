use smeg_kernel::mem::*;

use crate::mem::*;

use super::SystemControlBlockMemoryAttributes;

// TODO: Some sort of accessor is required - the #[arm_register] attribute will define it - but the structure and implementation are yet to be decided
pub struct IcsrAccessor<'mem> {
    accessor: CellAccessor<'mem, ReadonlyCell<SystemControlBlockMemoryAttributes, Icsr>>
}

#[mmio_register]
#[datasheet("DDI0403E.e", "B3.2.4", 599)]
#[rw(NMIPENDSET,  0b1_00_0_0_0_0_0_0_0_0_000000000_0_00_000000000)]
#[rw(PENDSVSET,   0b0_00_1_0_0_0_0_0_0_0_000000000_0_00_000000000)]
#[wo(PENDSVCLR,   0b0_00_0_1_0_0_0_0_0_0_000000000_0_00_000000000)]
#[rw(PENDSTSET,   0b0_00_0_0_1_0_0_0_0_0_000000000_0_00_000000000)]
#[wo(PENDSTCLR,   0b0_00_0_0_0_1_0_0_0_0_000000000_0_00_000000000)]
#[ro(ISRPREEMPT,  0b0_00_0_0_0_0_0_1_0_0_000000000_0_00_000000000)]
#[ro(ISRPENDING,  0b0_00_0_0_0_0_0_0_1_0_000000000_0_00_000000000)]
#[ro(VECTPENDING, 0b0_00_0_0_0_0_0_0_0_0_111111111_0_00_000000000)]
#[ro(RETTOBASE,   0b0_00_0_0_0_0_0_0_0_0_000000000_1_00_000000000)]
#[ro(VECTACTIVE,  0b0_00_0_0_0_0_0_0_0_0_000000000_0_00_111111111)]
#[xx(UNK_SBZP,    0b0_11_0_0_0_0_1_0_0_1_000000000_0_11_000000000)]
pub struct Icsr(u32);
