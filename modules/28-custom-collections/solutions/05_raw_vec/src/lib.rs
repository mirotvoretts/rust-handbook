//! 05 (2x) - растущий сырой буфер. Эталонное решение.

use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::ptr::NonNull;

pub struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    pub fn new() -> Self {
        assert_ne!(std::mem::size_of::<T>(), 0, "RawVec не поддерживает ZST");
        RawVec {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn ptr(&self) -> NonNull<T> {
        self.ptr
    }

    pub fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).expect("превышен предел ёмкости");

        let new_ptr = if self.cap == 0 {
            // SAFETY: new_layout ненулевого размера (T не ZST, new_cap > 0).
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: ptr/old_layout - от прошлого выделения этим же аллокатором.
            unsafe { realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Default for RawVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: ptr/layout - от выделения этим аллокатором; освобождаем один раз.
            unsafe { dealloc(self.ptr.as_ptr() as *mut u8, layout) };
        }
    }
}
