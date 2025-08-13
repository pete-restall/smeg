use smeg_kernel::McuSingleCore;
use smeg_kernel::bootstrapping::kernel::McuCoreBootstrapping;

#[derive(Default)]
pub struct McuCoreBootstrapper;

impl McuCoreBootstrapping for McuCoreBootstrapper {
    type McuCoreId = Self;
}

impl McuSingleCore for McuCoreBootstrapper { }
