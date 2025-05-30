use smeg_kernel::bootstrapping::kernel::McuCoreBootstrapping;

use crate::mcu_core::McuCore;

pub struct McuCoreBootstrapper;

impl McuCoreBootstrapping for McuCoreBootstrapper {
    type McuCoreId = McuCore;
}
