use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

use crate::errors::Result;
use crate::interrupts::IsrContext;

use super::{Lock, LockGuard, TryLock};

pub struct NoLock<T> {
    value: UnsafeCell<T>
}

impl<T> NoLock<T> {
    pub const fn new(value: T) -> Self where T: Default {
        Self { value: UnsafeCell::new(value) }
    }

    pub fn into_inner(self) -> Result<T> {
        Ok(self.value.into_inner())
    }

    pub fn get_mut(&mut self) -> Result<&mut T> {
        Ok(self.value.get_mut())
    }
}

impl<'protected, T: 'protected> Lock<'protected, T> for NoLock<T> {
    type Guard = NoLockGuard<'protected, T>;

    fn lock(&'protected self) -> Result<Self::Guard> {
        Ok(NoLockGuard { lock: self })
    }
}

impl<'protected, T: 'protected> TryLock<'protected, T> for NoLock<T> {
    type Guard = NoLockGuard<'protected, T>;

    fn try_lock(&'protected self) -> Result<Self::Guard> {
        Ok(NoLockGuard { lock: self })
    }
}

pub struct NoLockGuard<'protected, T> {
    lock: &'protected NoLock<T>
}

impl<T> LockGuard<'_, T> for NoLockGuard<'_, T> { }

impl<T> IsrContext for NoLockGuard<'_, T> { }

impl<T> Deref for NoLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for NoLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> !Send for NoLockGuard<'_, T> { }

unsafe impl<T: Sync> Sync for NoLockGuard<'_, T> { }

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use smeg_testing_host_utils::integers::{any_isize, any_usize, any_u8};

    use fluent_test::prelude::*;

    use super::*;

    mod no_lock_tests {
        use super::*;

        #[test]
        fn value__get__expect_same_value_passed_to_constructor() {
            let original_value = any_usize();
            let value = original_value;
            let lock = NoLock::new(value);
            expect!(lock.value.into_inner()).to_equal(original_value);
        }

        #[test]
        fn into_inner__called__expect_same_value_passed_to_constructor() {
            let original_value = any_isize();
            let value = original_value;
            let lock = NoLock::new(value);
            expect!(lock.into_inner().unwrap()).to_equal(original_value);
        }

        #[test]
        fn get_mut__called__expect_mutable_reference_to_value() {
            let new_value = any_usize();
            let mut lock = NoLock::new(any_usize());
            *lock.get_mut().unwrap() = new_value;
            expect!(lock.value.into_inner()).to_equal(new_value);
        }

        #[test]
        fn lock__called__expect_guard_with_reference_to_self() {
            let lock = NoLock::new(any_usize());
            let guard = lock.lock().unwrap();
            expect!(&raw const *guard.lock).to_equal(&raw const lock);
        }

        #[test]
        fn lock__called_multiple_times__expect_another_guard_with_reference_to_self_without_blocking() {
            let lock = NoLock::new(any_usize());
            let _first_guard = lock.lock().unwrap();
            let second_guard = lock.lock().unwrap();
            expect!(&raw const *second_guard.lock).to_equal(&raw const lock);
        }

        #[test]
        fn try_lock__called__expect_guard_with_reference_to_self() {
            let lock = NoLock::new(any_u8());
            let guard = lock.try_lock().unwrap();
            expect!(&raw const *guard.lock).to_equal(&raw const lock);
        }

        #[test]
        fn try_lock__called_multiple_times__expect_another_guard_with_reference_to_self_without_blocking() {
            let lock = NoLock::new(any_usize());
            let _first_guard = lock.try_lock().unwrap();
            let second_guard = lock.try_lock().unwrap();
            expect!(&raw const *second_guard.lock).to_equal(&raw const lock);
        }
    }

    mod no_lock_guard_tests {
        use core::ops::{Deref, DerefMut};

        use super::*;

        #[test]
        fn deref__called__expect_reference_to_lock_value() {
            let lock = NoLock::new(any_u8());
            let guard = lock.lock().unwrap();
            expect!(&raw const *guard.deref()).to_equal(lock.value.get());
        }

        #[test]
        fn deref_mut__called__expect_reference_to_lock_value() {
            let lock = NoLock::new(any_isize());
            let mut guard = lock.lock().unwrap();
            expect!(&raw mut *guard.deref_mut()).to_equal(lock.value.get());
        }
    }
}
