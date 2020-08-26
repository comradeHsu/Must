use std::cell::UnsafeCell;

pub struct RawPtr<T:?Sized> {
    data: UnsafeCell<T>
}

unsafe impl<T: ?Sized + Send> Send for RawPtr<T> {}

unsafe impl<T: ?Sized + Send> Sync for RawPtr<T> {}

impl<T> RawPtr<T> {

    pub fn new(t: T) -> RawPtr<T> {
        RawPtr {
            data: UnsafeCell::new(t),
        }
    }

    pub fn read(&self) -> &T {
        unsafe { &*self.data.get() }
    }

    pub fn write(&self) -> &mut T {
        unsafe { &mut *self.data.get() }
    }
}