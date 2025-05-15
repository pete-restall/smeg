use smeg_kernel::HasMcuCoreId;
use smeg_kernel::bootstrapping::kernel::McuCoreBootstrapping;

use crate::mcu_core::McuCore;

pub struct McuCoreBootstrapper;

impl McuCoreBootstrapping for McuCoreBootstrapper {
}

impl HasMcuCoreId for McuCoreBootstrapper {
    fn core_id() -> usize {
        McuCore::this_core_id()
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use smeg_testing_host_utils::integers::any_usize;
    use super::*;

    #[test]
    fn core_id__called__expect_id_from_thread_local_storage() {
        let core_id = any_usize();
        crate::mcu_core::tests::stub_this_core_id(core_id);
        expect!(McuCoreBootstrapper::core_id()).to_equal(core_id);
    }
}
