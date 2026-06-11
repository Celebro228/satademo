use core::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

pub struct Single<T> {
    inner: UnsafeCell<T>,
}

impl<T> Single<T> {
    pub const fn new(val: T) -> Self {
        Self {
            inner: UnsafeCell::new(val),
        }
    }

    pub fn get(&self) -> &T {
        unsafe { &*self.inner.get() }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    pub fn set(&self, val: T) {
        *self.get_mut() = val;
    }
}

impl<T> Deref for Single<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.get()
    }
}

impl<T> DerefMut for Single<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

unsafe impl<T> Sync for Single<T> {}
unsafe impl<T> Send for Single<T> {}

#[macro_export]
macro_rules! singleton {
    ($name:ident, $ty:ty, $init:expr) => {
        pub static $name: $crate::singleton::Single<$ty> = $crate::singleton::Single::new($init);
    };
}

pub struct LazySingle<T, F = fn() -> T> {
    cell: UnsafeCell<Option<T>>,
    init: F,
}

impl<T, F: Fn() -> T> LazySingle<T, F> {
    pub const fn new(init: F) -> Self {
        Self {
            cell: UnsafeCell::new(None),
            init,
        }
    }

    pub fn get(&self) -> &T {
        let opt = unsafe { &mut *self.cell.get() };
        if opt.is_none() {
            *opt = Some((self.init)());
        }
        opt.as_ref().unwrap()
    }

    pub fn get_mut(&self) -> &mut T {
        let opt = unsafe { &mut *self.cell.get() };
        if opt.is_none() {
            *opt = Some((self.init)());
        }
        opt.as_mut().unwrap()
    }
}

unsafe impl<T, F> Sync for LazySingle<T, F> {}
unsafe impl<T, F> Send for LazySingle<T, F> {}