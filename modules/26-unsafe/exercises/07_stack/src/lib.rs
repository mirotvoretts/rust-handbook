//! 07 (3x) - стек фиксированной ёмкости на `MaybeUninit` с корректным `Drop`.
//!
//! Реализуй `Stack<T>` - стек ёмкостью `Stack::<T>::CAP` (== 8) без кучи: хранилище -
//! массив `[MaybeUninit<T>; 8]`, а `len` считает, сколько ячеек с начала уже
//! инициализировано. Это и есть инвариант типа: ячейки `[0, len)` содержат валидные
//! `T`, ячейки `[len, 8)` не инициализированы. Все методы обязаны его поддерживать.
//!
//! API:
//! - `Stack::new()` - пустой стек;
//! - `push(&mut self, value) -> Result<(), T>` - положить сверху; если стек полон
//!   (`len == CAP`), вернуть `Err(value)`, не теряя значение;
//! - `pop(&mut self) -> Option<T>` - снять верхний по владению или `None`, если пуст;
//! - `len(&self) -> usize`, `is_empty(&self) -> bool`.
//!
//! Обязательно реализуй `Drop`: при уничтожении стека нужно дропнуть ровно `len`
//! инициализированных элементов (и ни одной неинициализированной ячейки), иначе -
//! утечка или чтение/дроп мусора (разделы 6-7 README).
//!
//! Конструкции за пределами теории:
//! - неинициализированный буфер: MaybeUninit (uninit/write/as_ptr/as_mut_ptr)
//!   https://doc.rust-lang.org/std/mem/union.MaybeUninit.html
//! - забрать значение по адресу: <*const T>::read
//!   https://doc.rust-lang.org/std/primitive.pointer.html#method.read
//! - дропнуть значение по адресу, не освобождая память: std::ptr::drop_in_place
//!   https://doc.rust-lang.org/std/ptr/fn.drop_in_place.html

use std::mem::MaybeUninit;

pub struct Stack<T> {
    _buf: [MaybeUninit<T>; 8],
    _len: usize,
}

impl<T> Stack<T> {
    pub const CAP: usize = 8;

    pub fn new() -> Self {
        todo!()
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        let _ = value;
        todo!()
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
}
