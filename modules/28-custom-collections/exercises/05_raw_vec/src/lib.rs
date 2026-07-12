//! 05 (2x) - растущий сырой буфер (раздел 5 README).
//!
//! `RawVec<T>` - это только буфер: указатель `ptr` и ёмкость `cap`. Он НЕ хранит длину и
//! НЕ роняет элементы; его задача - выделять и наращивать память под будущие элементы.
//! На нём в упр. 06 строится настоящий `MyVec`. Реализуй управление ростом.
//!
//! Тип объявлен: `RawVec<T> { ptr: NonNull<T>, cap: usize }`. Считаем `T` ненулевого
//! размера (ZST - упр. 08).
//!
//! - `new()` -> `Self`: пустой буфер без выделения. Ёмкость 0, а `ptr` - "висячий, но
//!   выровненный" указатель `NonNull::dangling()` (пока `cap == 0`, разыменовывать его
//!   нельзя, но он не null и годится как заглушка).
//! - `capacity()` -> `usize`, `ptr()` -> `NonNull<T>` - геттеры.
//! - `grow(&mut self)`: увеличить ёмкость по схеме "0 -> 4, иначе удвоение"
//!   (`new_cap = if cap == 0 { 4 } else { cap * 2 }`):
//!     - собери `Layout::array::<T>(new_cap)` (при переполнении - паника, это ок);
//!     - если старая ёмкость 0 - выдели заново (`alloc(new_layout)`); иначе перевыдели
//!       (`realloc(old_ptr as *mut u8, old_layout, new_layout.size())`), где `old_layout`
//!       посчитан для старой ёмкости;
//!     - при нулевом результате - `handle_alloc_error(new_layout)`; иначе обнови `ptr`
//!       (через `NonNull::new`) и `cap`.
//! - `Drop`: если `cap != 0`, освободить буфер (`dealloc` с `Layout::array::<T>(cap)`).
//!   Элементы не роняем - это забота владельца буфера.
//!
//! `realloc` копирует старое содержимое в новый блок сам; отдельно копировать байты не
//! нужно. Документация:
//! <https://doc.rust-lang.org/std/alloc/fn.realloc.html>,
//! `NonNull::dangling`: <https://doc.rust-lang.org/std/ptr/struct.NonNull.html#method.dangling>.

use std::ptr::NonNull;

pub struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        todo!()
    }

    pub fn ptr(&self) -> NonNull<T> {
        todo!()
    }

    pub fn grow(&mut self) {
        todo!()
    }
}

impl<T> Default for RawVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        todo!()
    }
}
