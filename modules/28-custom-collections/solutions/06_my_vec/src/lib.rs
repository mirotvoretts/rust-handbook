//! 06 (2x) - свой `Vec<T>`. Эталонное решение.

use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        assert_ne!(std::mem::size_of::<T>(), 0, "MyVec не поддерживает ZST");
        MyVec {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    fn grow(&mut self) {
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

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow();
        }
        // SAFETY: ptr.add(len) - в пределах буфера (len < cap после grow); write
        // инициализирует ранее незанятую ячейку без drop старого содержимого.
        unsafe { self.ptr.as_ptr().add(self.len).write(value) };
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: ячейка [len] инициализирована; read забирает значение, дальше len
        // указывает на неё как на свободную - повторного чтения не будет.
        Some(unsafe { self.ptr.as_ptr().add(self.len).read() })
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        if i < self.len {
            // SAFETY: i < len => ячейка инициализирована и жива.
            Some(unsafe { &*self.ptr.as_ptr().add(i) })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if i < self.len {
            // SAFETY: i < len, и &mut self даёт эксклюзивный доступ.
            Some(unsafe { &mut *self.ptr.as_ptr().add(i) })
        } else {
            None
        }
    }
}

impl<T> Default for MyVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        // SAFETY: первые len ячеек инициализированы и лежат подряд.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        // SAFETY: как выше; &mut self даёт эксклюзивный доступ к len элементам.
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        // Сначала роняем все живые элементы...
        while self.pop().is_some() {}
        // ...затем освобождаем сам буфер.
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: ptr/layout - от нашего выделения; освобождаем один раз.
            unsafe { dealloc(self.ptr.as_ptr() as *mut u8, layout) };
        }
    }
}
