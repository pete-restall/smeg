mod interrupted_task;
pub use interrupted_task::*;

mod task_scheduler;
pub use task_scheduler::*;

pub trait Task { }
