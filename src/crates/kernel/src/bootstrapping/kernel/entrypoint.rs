use crate::HasMcuCoreId;
use crate::bootstrapping::kernel::{BoardMcuBootstrapping, McuCoreBootstrapping};
use crate::bootstrapping::rust::RuntimeBootstrapping;

pub unsafe trait Entrypoint {
    type RuntimeBootstrapper: RuntimeBootstrapping;
    type McuCoreBootstrapper: McuCoreBootstrapping;
    type BoardMcuBootstrapper: BoardMcuBootstrapping;

    unsafe fn entrypoint() -> ! {
        // TODO: roll this into the kernel as a default implementation on the Entrypoint trait
        unsafe {
            if Self::McuCoreBootstrapper::core_id() == 0 {
                Self::RuntimeBootstrapper::bootstrap();
            }
        }

        // TODO:
        // Where now...?  At this point we have the runtime initialised.  We want to reset the stack pointer and invoke the scheduler with the
        // kernel's initialisation task and pass in any injected types (eg. factories, the BoardMcuBootstrapper, etc.)
        loop { }
    }
}
