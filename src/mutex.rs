use core::cell::UnsafeCell;
use core::ops::Deref;
use core::ops::DerefMut;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

use ruduino::interrupt::without_interrupts;

pub struct Mutex<T> {
    is_locked: AtomicBool,
    data: T,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> MutexGuard<'a, T> {
    fn new(mutex: &'a Mutex<T>) -> MutexGuard<'a, T> {
        MutexGuard { mutex }
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.mutex.data
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut (*(self.mutex as *const Mutex<T> as *mut Mutex<T>)).data }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.is_locked.store(false, Ordering::Release);
    }
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Mutex<T> {
        Mutex {
            is_locked: AtomicBool::new(false),
            data: value,
        }
    }

    pub fn lock(&self) -> Option<MutexGuard<T>> {
        without_interrupts(|| {
            if self.is_locked.load(Ordering::Acquire) {
                return None;
            }

            self.is_locked.store(true, Ordering::Release);

            Some(MutexGuard::new(self))
        })
    }
}
