use super::{BoardMcuBootstrapping, McuCoreBootstrapping};

pub fn entrypoint<C: McuCoreBootstrapping, B: BoardMcuBootstrapping>() -> ! {
    // TODO: where it all happens...
    loop { }
}
