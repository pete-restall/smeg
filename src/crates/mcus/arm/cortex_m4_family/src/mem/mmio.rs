use smeg_kernel::ConstUsize;
use smeg_kernel::mem::{Cell, CellAccessor, Readable};

use super::{MemoryShareability, MemorySideEffects, MemoryType, NoSideEffects, NotShareable, StronglyOrderedMemory};

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


pub unsafe trait MmioRead<T: Copy> { // TODO: better name ?  AtomicVolatileRead ?  Because we want other traits, such as LlscReadModifyWrite (based on LDREX / STREX, configurable retries)
     unsafe fn mmio_read(&self) -> T;
}

pub unsafe trait MmioReadMasked<T: Copy> {
    unsafe fn mmio_read_masked_shifted_left<const MASK: usize>(&self) -> T;
    unsafe fn mmio_read_masked_shifted_right<const MASK: usize>(&self) -> T;
    unsafe fn mmio_read_masked<const MASK: usize>(&self) -> T;
}

unsafe impl<T> MmioReadMasked<usize> for T where T: MmioRead<usize> {
    unsafe fn mmio_read_masked_shifted_right<const MASK: usize>(&self) -> usize {
        0
        // TODO: something like...
        // let value = unsafe { self.mmio_read_masked::<MASK>() };
        // if MASK == 0 { 0 } else { value >> ConstUsize::<MASK>::count_trailing_zeroes() }
    }

    unsafe fn mmio_read_masked<const MASK: usize>(&self) -> usize {
        unsafe { self.mmio_read() & MASK } // TODO: VERIFY THAT THIS DOES NOT GET OPTIMISED AWAY WHEN MASK == 0 !
    }

    unsafe fn mmio_read_masked_shifted_left<const MASK: usize>(&self) -> usize {
        0
        // TODO: something like...
        // let value = unsafe { self.mmio_read_masked::<MASK>() };
        // if MASK == 0 { 0 } else { value << ConstUsize::<MASK>::count_trailing_zeroes() }
    }
}

#[cfg(not(any(test, feature = "test_doubles")))]
unsafe impl<'mem, C, M> MmioRead<usize> for CellAccessor<'mem, C> where
    C: Readable + Cell<MemoryAttributes = M, Type = usize>,
    M:
        MemoryType<Type = StronglyOrderedMemory> +
        MemoryShareability<Shareability = NotShareable> +
        MemorySideEffects<NormalMemory = NoSideEffects> {

    unsafe fn mmio_read(&self) -> usize {
        #[allow(unused_mut)] let mut value;
        unsafe {
            cfg_if::cfg_if! {
                if #[cfg(target_arch = "arm")] {
                    use core::arch::asm;
                    let addr = self.get() as usize;
                    asm!(
                        "ldr {value}, [{addr}]",
                        addr = in(reg) addr,
                        value = lateout(reg) value,
                        options(preserves_flags, nostack));

                } else {
                    panic!("Cannot use Cortex M4 assembly language on a non-Cortex M4 (running tests ?)");
                }
            }
        }
        value
    }
}

#[cfg(any(test, feature = "test_doubles"))] // TODO: make this into a test double, etc.
unsafe impl<'mem, C> MmioRead<usize> for CellAccessor<'mem, C> where C: Readable + Cell<Type = usize> {
    unsafe fn mmio_read(&self) -> usize {
        0
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    // TODO...tests
}
