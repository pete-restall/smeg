use super::Task;

pub trait HasInterruptedTask {
    type InterruptedTask: Task;

    fn interrupted_task<'a>(&'a self) -> Option<&'a Self::InterruptedTask>;
}

pub trait HasInterruptedTaskMut: HasInterruptedTask {
    fn interrupted_task_mut<'a>(&'a mut self) -> Option<&'a mut Self::InterruptedTask>;
}
