//! 07 (3x) - стек фиксированной ёмкости на `MaybeUninit` с корректным `Drop`.
//! Эталонное решение.

use std::mem::MaybeUninit;
use std::ptr;

pub struct Stack<T> {
    // инвариант: ячейки [0, len) инициализированы, [len, CAP) - нет.
    buf: [MaybeUninit<T>; 8],
    len: usize,
}

impl<T> Stack<T> {
    pub const CAP: usize = 8;

    pub fn new() -> Self {
        Self {
            buf: [const { MaybeUninit::uninit() }; 8],
            len: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len == Self::CAP {
            return Err(value); // полон - возвращаем значение, не теряя его
        }
        self.buf[self.len].write(value); // инициализируем следующую ячейку
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: ячейка len была инициализирована push'ем; забираем по владению
        // и логически считаем её больше не инициализированной (len уменьшен).
        Some(unsafe { self.buf[self.len].as_ptr().read() })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        for slot in &mut self.buf[..self.len] {
            // SAFETY: дропаем только инициализированные ячейки [0, len).
            unsafe { ptr::drop_in_place(slot.as_mut_ptr()) };
        }
    }
}
