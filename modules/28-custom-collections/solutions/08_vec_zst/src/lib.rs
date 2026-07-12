//! 08 (3x) - вектор из типов нулевого размера. Эталонное решение.

use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct ZstVec<T> {
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> ZstVec<T> {
    pub fn new() -> Self {
        assert_eq!(
            std::mem::size_of::<T>(),
            0,
            "ZstVec поддерживает только типы нулевого размера"
        );
        ZstVec {
            len: 0,
            _marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        usize::MAX
    }

    pub fn push(&mut self, value: T) {
        // Памяти под ZST нет; ронять сейчас нельзя - забываем значение без деструктора,
        // а его "экземпляр" воссоздаст pop/Drop.
        std::mem::forget(value);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: T нулевого размера, чтение из dangling не трогает ни одного байта;
        // результат - владеемый T, чей деструктор отработает у вызывающего.
        Some(unsafe { NonNull::<T>::dangling().as_ptr().read() })
    }
}

impl<T> Default for ZstVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for ZstVec<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
