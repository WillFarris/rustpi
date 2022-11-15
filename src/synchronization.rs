use core::cell::UnsafeCell;

pub mod interface {
    use core::ops::{Deref, DerefMut};

    pub trait Mutex {
        type Data;

        //fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R;
        fn lock(&self) -> Result<MutexGuard<Self>, ()>;
        fn unlock(&self) -> Result<(), ()>;
        unsafe fn get_data(&self) -> &Self::Data;
        unsafe fn get_data_mut(&self) -> &mut Self::Data;
    }

    pub struct MutexGuard<'a, T: ?Sized + Mutex> {
        inner: &'a T,
    }
    
    impl<'a, T: ?Sized + Mutex> MutexGuard<'a, T> {
        pub fn new(inner: &'a T) -> Self {
            Self {
                inner
            }
        }
    }

    impl<T: ?Sized + Mutex> Drop for MutexGuard<'_, T> {
        fn drop(&mut self) {
            self.inner.unlock().unwrap();
        }
    }

    impl<T: ?Sized + Mutex> Deref for MutexGuard<'_, T> {
        type Target = T::Data;

        fn deref(&self) -> &Self::Target {
            unsafe {
                self.inner.get_data()
            }
        }
    }

    impl<T: ?Sized + Mutex> DerefMut for MutexGuard<'_, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                self.inner.get_data_mut()
            }
        }
    }

}

pub struct SpinLock<T> where T: ?Sized {
    guard: u8,
    data: UnsafeCell<T>,
}

unsafe impl<T> Send for SpinLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for SpinLock<T> where T: ?Sized + Send {}

impl<T> SpinLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            guard: 0,
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> interface::Mutex for SpinLock<T> {
    type Data = T;

    /*
    fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R {
        let data = unsafe {&mut *self.data.get() };

        f(data)
    }
    */

    fn lock(&self) -> Result<interface::MutexGuard<Self>, ()> {
        
        let one = 1;
        let mut guard = 0;
        let mut set_result = 0;
        while guard != 0 && set_result != 0 {
            unsafe {
                core::arch::asm!("
                    ldaxrb {g:w}, [{x}]
                    stlxrb {s:w}, {o:w}, [{x}]
                ",
                g = out(reg) guard,
                s = out(reg) set_result,
                o = in(reg) one,
                x = in(reg) (&self.guard as *const u8 as usize));
            }
        }

        Ok(interface::MutexGuard::new(self))
    }

    fn unlock(&self) -> Result<(), ()> {
        unsafe {
            core::arch::asm!("
                stlr   wzr, [{x}]
            ", x = in(reg) (&self.guard as *const u8 as usize));
        }
        
        Ok(())
    }

    unsafe fn get_data(&self) -> &Self::Data {
        & *self.data.get()
    }

    unsafe fn get_data_mut(&self) -> &mut Self::Data {
        &mut *self.data.get()
    }
}
