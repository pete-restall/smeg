use smeg_kernel::mem::*;

use crate::mem::*;

#[repr(C)] // TODO: this will be applied by the derivation / attribute
// Something like this maybe: #[arm_register_bank(StronglyOrderedMemory, NotShareable)]
struct SystemControlBlock {
    cpuid: ReadonlyCell<SystemControlBlockMemoryAttributes, Cpuid>, // #[Ro] cpuid: Cpuid
    icsr: ReadWriteCell<SystemControlBlockMemoryAttributes, Icsr> // #[Rw] icsr: Icsr
}

struct SystemControlBlockMemoryAttributes;
unsafe impl MemoryAttributes for SystemControlBlockMemoryAttributes { }
unsafe impl MemoryType for SystemControlBlockMemoryAttributes { type Type = StronglyOrderedMemory; }
unsafe impl MemoryShareability for SystemControlBlockMemoryAttributes { type Shareability = NotShareable; }
unsafe impl MemorySideEffects for SystemControlBlockMemoryAttributes { type NormalMemory = NoSideEffects; }

// also from derive
// impl Bank for SystemControlBlock { }
// etc.

mod cpuid;
pub use cpuid::*;

mod icsr;
pub use icsr::*;
