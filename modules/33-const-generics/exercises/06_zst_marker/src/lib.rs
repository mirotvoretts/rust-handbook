//! 06 (2x) - типы нулевого размера на практике.
//!
//! Разбери типы нулевого размера (ZST). Требуемое API:
//! - `is_zst<T>() -> bool` (const fn) - истина, если `size_of::<T>() == 0`;
//! - ZST-метка `pub struct Marker;`;
//! - множество `TinySet<T>` поверх `HashMap<T, ()>`: `new`, `insert` (возвращает
//!   `true`, если элемент новый), `contains`, `len`, `is_empty`;
//! - счётчик `UnitCounter` поверх `Vec<()>`: `new`, `tick`, `count`.
//!
//! Подсказки:
//! - `insert` у `HashMap` возвращает прежнее значение; новизну определяет
//!   `.is_none()`;
//! - тело `todo!()` в `const fn is_zst` недопустимо, поэтому заглушка возвращает
//!   `false` - тесты падают на утверждениях, пока не решено.
//!
//! Про ZST:
//! <https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts>.
use std::collections::HashMap;
use std::hash::Hash;

pub struct Marker;

pub const fn is_zst<T>() -> bool {
    false
}

pub struct TinySet<T> {
    map: HashMap<T, ()>,
}

impl<T: Hash + Eq> TinySet<T> {
    pub fn new() -> Self {
        todo!()
    }
    pub fn insert(&mut self, value: T) -> bool {
        let _ = value;
        todo!()
    }
    pub fn contains(&self, value: &T) -> bool {
        let _ = value;
        todo!()
    }
    pub fn len(&self) -> usize {
        todo!()
    }
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}

pub struct UnitCounter {
    items: Vec<()>,
}

impl UnitCounter {
    pub fn new() -> Self {
        todo!()
    }
    pub fn tick(&mut self) {
        todo!()
    }
    pub fn count(&self) -> usize {
        todo!()
    }
}
