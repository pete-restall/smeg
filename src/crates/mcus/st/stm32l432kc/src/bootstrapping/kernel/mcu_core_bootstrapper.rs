use smeg_kernel::HasMcuCoreId;
use smeg_kernel::bootstrapping::kernel::McuCoreBootstrapping;

pub struct McuCoreBootstrapper;

impl McuCoreBootstrapping for McuCoreBootstrapper {
}

impl HasMcuCoreId for McuCoreBootstrapper {
    fn core_id() -> usize {
        0
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use fluent_test::prelude::*;

    use super::*;

    #[test]
    fn core_id__called__expect_hard_coded_zero() {
        expect!(McuCoreBootstrapper::core_id()).to_equal(0);
    }
}
