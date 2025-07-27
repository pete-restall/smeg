use smeg_kernel::bootstrapping::kernel::BoardMcuBootstrapping;

pub struct BoardMcuBootstrapper;

// TODO: these need to be implemented properly for the host.  The host will essentially require a stub ISR vector table.
pub struct DummyToDo;

impl smeg_kernel::interrupts::IsrContext for DummyToDo {
    type Mcu = smeg_kernel::McuSingleCore; // TODO: obviously not...
}

impl BoardMcuBootstrapping for BoardMcuBootstrapper {
}
