use smeg_kernel::bootstrapping::rust::*;

pub struct McuMemoryBootstrapper;

unsafe impl McuMemoryBootstrapping for McuMemoryBootstrapper {
    unsafe fn bootstrap_bss_sections_using<I: BssSectionInitialisation>() {
        /* This is done by libc for the hosts relying on std */
    }

    unsafe fn bootstrap_data_sections_using<I: DataSectionInitialisation>() {
        /* This is done by libc for the hosts relying on std */
    }
}
