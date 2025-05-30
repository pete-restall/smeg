use smeg_kernel::McuSingleCore;
use smeg_kernel::bootstrapping::kernel::McuCoreBootstrapping;

pub struct McuCoreBootstrapper;

impl McuCoreBootstrapping for McuCoreBootstrapper {
    type McuCoreId = McuSingleCore;
}
