//! 04 (1x) - свой `Box<T>`. Эталонное решение.

use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct MyBox<T> {
    ptr: NonNull<T>,
}

impl<T> MyBox<T> {
    pub fn new(value: T) -> Self {
        assert_ne!(std::mem::size_of::<T>(), 0, "MyBox не поддерживает ZST");
        let layout = Layout::new::<T>();
        // SAFETY: layout ненулевого размера (проверено assert выше).
        let raw = unsafe { alloc(layout) } as *mut T;
        let ptr = match NonNull::new(raw) {
            Some(p) => p,
            None => handle_alloc_error(layout),
        };
        // SAFETY: свежая выровненная память; write инициализирует её без drop старого.
        unsafe { ptr.as_ptr().write(value) };
        MyBox { ptr }
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: ptr всегда ведёт на инициализированный, живой T.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: &mut self даёт эксклюзивный доступ к значению.
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // SAFETY: сначала запускаем деструктор значения, затем освобождаем его память.
        unsafe {
            std::ptr::drop_in_place(self.ptr.as_ptr());
            dealloc(self.ptr.as_ptr() as *mut u8, Layout::new::<T>());
        }
    }
}
