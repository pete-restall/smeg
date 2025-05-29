use core::mem::MaybeUninit;

use smeg_kernel::bootstrapping::rust::*;

pub struct McuMemoryBootstrapper;

unsafe impl McuMemoryBootstrapping for McuMemoryBootstrapper {
    #[allow(static_mut_refs)]
    unsafe fn bootstrap_bss_sections_using<I: BssSectionInitialisation>() {
        unsafe extern "C" {
            unsafe static mut __LINKER_BSS_START: MaybeUninit<usize>;
            unsafe static __LINKER_BSS_PAST_END: MaybeUninit<usize>;
        }

        unsafe {
            I::fill_bss_section(&mut __LINKER_BSS_START, &__LINKER_BSS_PAST_END, 0x00);
        }
    }

    #[allow(static_mut_refs)]
    unsafe fn bootstrap_data_sections_using<I: DataSectionInitialisation>() {
        unsafe extern "C" {
            unsafe static mut __LINKER_DATA_START: MaybeUninit<usize>;
            unsafe static __LINKER_DATA_PAST_END: MaybeUninit<usize>;
            unsafe static __LINKER_DATA_LMA_START: usize;
        }

        unsafe {
            I::load_data_section(&mut __LINKER_DATA_START, &__LINKER_DATA_PAST_END, &__LINKER_DATA_LMA_START);
        }
    }
}
