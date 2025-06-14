use smeg_kernel::bootstrapping::kernel::BoardMcuBootstrapping;

pub struct BoardMcuBootstrapper;

// TODO: these need to be implemented properly for the host.  The host will essentially require a stub ISR vector table.
pub struct DummyToDo;
impl smeg_kernel::bootstrapping::kernel::IsrBootstrapping for DummyToDo {
    type IsrContext = DummyToDo;
}
impl smeg_kernel::interrupts::IsrContext for DummyToDo {
}

impl BoardMcuBootstrapping for BoardMcuBootstrapper {
    type IsrBootstrapper = DummyToDo;
}
