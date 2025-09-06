use crate::errors::Result;

mod no_lock;
pub use no_lock::*;

pub trait LockGuard<'protected, T> { }

pub trait Lock<'protected, T> {
	type Guard: LockGuard<'protected, T>;

    fn lock(&'protected self) -> Result<Self::Guard>;
}

pub trait TryLock<'protected, T> {
	type Guard: LockGuard<'protected, T>;

    fn try_lock(&'protected self) -> Result<Self::Guard>;
}
