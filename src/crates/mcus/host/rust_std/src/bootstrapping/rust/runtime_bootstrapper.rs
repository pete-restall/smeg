use smeg_kernel::bootstrapping::rust::*;

pub struct RuntimeBootstrapper;

unsafe impl RuntimeBootstrapping for RuntimeBootstrapper {
    unsafe fn initialise_bss_sections_using<I: BssSectionInitialiser>(_initialiser: &I) {
        /* This is done by libc for the hosts relying on std */
    }

    unsafe fn initialise_data_sections_using<I: DataSectionInitialiser>(_initialiser: &I) {
        /* This is done by libc for the hosts relying on std */
    }
}
