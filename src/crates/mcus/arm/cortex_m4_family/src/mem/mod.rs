use smeg_kernel::mem::MemoryAttributes;

mod mmio;
pub use mmio::*;

pub unsafe trait MemoryType: MemoryAttributes {
    type Type;
}

pub enum NormalMemory { }
pub enum DeviceMemory { }
pub enum StronglyOrderedMemory { }

pub unsafe trait MemoryShareability: MemoryAttributes {
    type Shareability; // p83, A3.5.5 of ARM v7 reference manual - device memory that is non-shareable, like the PPB on the (ARMv6) RP2040
}

pub enum Shareable { }
pub enum NotShareable { }

pub unsafe trait MemorySideEffects: MemoryAttributes {
    type NormalMemory; // if NoSideEffects then no memory barriers are required; if HasSideEffects then memory barriers required.  See p82, A3.5.5 of ARM v7 reference manual
}

pub enum SideEffects { }
pub enum NoSideEffects { }
