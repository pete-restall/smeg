use core::mem::MaybeUninit;

use smeg_kernel::bootstrapping::rust::*;

pub struct RuntimeBootstrapper;

unsafe impl RuntimeBootstrapping for RuntimeBootstrapper {
    #[allow(static_mut_refs)]
    unsafe fn initialise_bss_sections_using<I: BssSectionInitialiser>(initialiser: &I) {
        unsafe extern "C" {
            unsafe static mut __LINKER_BSS_START: MaybeUninit<usize>;
            unsafe static __LINKER_BSS_PAST_END: MaybeUninit<usize>;
        }

        unsafe {
            initialiser.fill_bss_section(&mut __LINKER_BSS_START, &__LINKER_BSS_PAST_END, 0x00);
        }
    }

    #[allow(static_mut_refs)]
    unsafe fn initialise_data_sections_using<I: DataSectionInitialiser>(initialiser: &I) {
        unsafe extern "C" {
            unsafe static mut __LINKER_DATA_START: MaybeUninit<usize>;
            unsafe static __LINKER_DATA_PAST_END: MaybeUninit<usize>;
            unsafe static __LINKER_DATA_LMA_START: usize;
        }

        unsafe {
            initialiser.load_data_section(&mut __LINKER_DATA_START, &__LINKER_DATA_PAST_END, &__LINKER_DATA_LMA_START);
        }
    }
}
