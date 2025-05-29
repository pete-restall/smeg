use smeg_kernel::bootstrapping::kernel::Entrypoint;

use super::board::bootstrapping::kernel as board;
use super::rust::Rust;

pub struct Kernel;

unsafe impl Entrypoint for Kernel {
    type RuntimeBootstrapper = Rust;
    type McuCoreBootstrapper = board::McuCoreBootstrapper;
    type BoardMcuBootstrapper = board::BoardMcuBootstrapper;
}
