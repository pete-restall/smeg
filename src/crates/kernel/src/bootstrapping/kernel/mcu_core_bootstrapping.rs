use crate::HasMcuCoreId;

pub trait McuCoreBootstrapping {
    type McuCoreId: HasMcuCoreId;
    // TODO: MCU core-specific bootstrapping stuff
}
