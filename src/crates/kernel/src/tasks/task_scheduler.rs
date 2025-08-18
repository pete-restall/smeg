pub trait TaskScheduler { }

pub trait HasTaskScheduler {
    type TaskScheduler;
}
