use core::mem::size_of;

use smeg_kernel::mem::{Addressable, Bank, BankAccessor, CellAccessor, MemoryAttributes, ReadWriteCell};

use crate::mem::{DeviceMemory, MemorySideEffects, MemoryShareability, MemoryType, NoSideEffects, Shareable, StronglyOrderedMemory};

pub mod scb;

pub struct SystemControlSpaceAccessor<'mem> {
    accessor: BankAccessor<'mem, SystemControlSpace>
}

impl<'mem> SystemControlSpaceAccessor<'mem> {
    cfg_if::cfg_if! {
        if #[cfg(any(test, feature = "test_doubles"))] {
            pub const unsafe fn new(bank: &'mem mut SystemControlSpace) -> Self {
                unsafe { Self { accessor: BankAccessor::new(&raw mut *bank) } }
            }
        } else {
            pub const unsafe fn new() -> Self {
                unsafe {
                    unsafe extern "C" { static mut __LINKER_ARM_PPB_SCS: SystemControlSpace; }
                    Self { accessor: BankAccessor::new(&raw mut __LINKER_ARM_PPB_SCS) }
                }
            }
        }
    }
}

#[repr(C)]
//#[derive(ArmRegisterBank(StronglyOrderedMemory, NotShareable)]
struct SystemControlSpace {
// TODO: The way this struct is defined at the moment doesn't really make sense, since the dependency arrows will end up pointing from here into the more
// nested namespaces, rather than the other way around.  What we probably want is a linker symbol for each bank (eg. SCB) and this namespace just defines
// the memory attributes of the region, etc.  Potentially also some traits that can be applied by the inner modules to the SystemControlSpaceAccessor so
// they can extend it and keep the arrows pointing in the right direction.
//
//    _test: ReadWriteCell<SystemControlSpaceMemoryAttributes, usize>,
    _x: [u8; 4096]
//	#[Rw] x: usize // --> ReadWriteCell<SystemControlSpaceMemoryAttributes, usize>
}

unsafe impl Bank for SystemControlSpace { }
unsafe impl Addressable for SystemControlSpace { type MemoryAttributes = SystemControlSpaceMemoryAttributes; }

struct SystemControlSpaceMemoryAttributes;
unsafe impl MemoryAttributes for SystemControlSpaceMemoryAttributes { }
unsafe impl MemoryType for SystemControlSpaceMemoryAttributes { type Type = StronglyOrderedMemory; }
unsafe impl MemoryShareability for SystemControlSpaceMemoryAttributes { type Shareability = Shareable; }

#[cfg(target_arch = "arm")]
const _: () = assert!(size_of::<SystemControlSpace>() == 4096, "[DDI0403E.e, B3.2] System Control Space Register Bank must be 4KiB");
/*
pub fn xxx<T>(x: &T) where T: SpecialArmRead {
    _ = x.special_read();

    // DSB will (explicitly) ensure SCS side-effects (p. A3-95 and A3-96) are seen before the next instruction executes - but a subsequent ISB is also
    // required to ensure ordering; not all SCS writes will require a write-DSB-ISB sequence, however

    // Can Rust compiler reorder asm! blocks in relation to other asm! blocks, or even the instructions surrounding the asm! blocks ?
    // Does this mean we need compiler_fence! around any asm! ?  Yes to reordering; probably no to compiler_fence! as that is only for memory, but
    // may be necessary.

    // To pend any interrupt, we need to set the PENDxx flag and then issue DSB+ISB; the NVIC exhibits DSB-like behaviour but only in relation to
    // Device and Strongly Ordered memory; if previous operations on Normal memory need to be observed by the ISR then the DSB is necessary, so it's
    // basically always necessary because an encapsulated 'pend this interrupt' function cannot know about the call-site it is inserted into.
    // Note that the 'DSB+ISB' is _after_ the PENDxx flag has been set because the pipeline could execute up to two instructions before the pending ISR
    // is called; no DSB is required beforehand since Cortex M architectures will have applied the memory operations in order prior to jumping to the ISR

    // Section 3.3 of [DAI0321A] 'rules of thumb' for architectural requirements around barriers

    // Section 3.3 of [DAI0321A] for spin-locks / semaphores; DMB is required for multi-master (eg. DMA, multi-core, etc.) systems, although section 3.5 says _all_ DMB is redundant for Cortex M3/M4

    // Section 4.4 - SCS Peripheral Access - no need for any memory barriers at all for Cortex M devices, since implicit DMB side-effect, IFF there are
    // no Normal memory accesses or the subsequent instruction does not require the effects to be visible immediately.  An example of an instruction that
    // needs to see the SCS side-effect immediately is updating, say, the SCR register and then executing WFI; eg. STR to SCR -> DSB -> WFI
}

trait SpecialArmRead {
    fn special_read(&self) -> usize;
}

impl<'mem, C, M> SpecialArmRead for smeg_kernel::mem::CellAccessor<'mem, C> where
    C:
        smeg_kernel::mem::Readable +
        smeg_kernel::mem::Cell<MemoryAttributes = M, Type = usize>,
    M:
        MemoryType<Type = DeviceMemory> +
        MemoryShareability<Shareability = Shareable> +
        MemorySideEffects<NormalMemory = NoSideEffects> {

    fn special_read(&self) -> usize {
        12345
    }
}
*/
/*
use crate::{NoSideEffects, NotShareable};

trait SpecialArmRead {
    fn special_read(&self) -> usize;
}

impl<'mem, M, T> SpecialArmRead for smeg_kernel::mem::Accessor<'mem, M, T> where
    T: smeg_kernel::mem::Read<'mem, M, usize>,
    M:
        smeg_kernel::mem::MemoryAttributes +
        crate::MemoryType<Type = crate::DeviceMemory> +
        crate::MemoryShareability<Shareability = crate::NotShareable> +
        crate::MemorySideEffects<NormalMemory = crate::NoSideEffects> {

    fn special_read(&self) -> usize {
        12345
    }
}
*/
