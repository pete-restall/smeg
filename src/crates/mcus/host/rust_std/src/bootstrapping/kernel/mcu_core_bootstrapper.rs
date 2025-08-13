use smeg_kernel::bootstrapping::{kernel::McuCoreBootstrapping, BootstrapperContext};

use crate::mcu_core::McuCore;

impl BootstrapperContext for McuCore { }

pub struct McuCoreBootstrapper;

impl McuCoreBootstrapping for McuCoreBootstrapper {
    type McuCoreId = McuCore;
}
