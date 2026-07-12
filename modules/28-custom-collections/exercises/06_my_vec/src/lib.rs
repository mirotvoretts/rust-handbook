//! 06 (2x) - свой `Vec<T>` (раздел 5 README).
//!
//! Центральное упражнение модуля: растущий вектор с корректным владением элементами.
//! Опирается на ту же схему роста, что в упр. 05, но теперь буфер частично заполнен -
//! появляется длина `len` (сколько элементов инициализировано; `len <= cap`).
//!
//! Тип объявлен: `MyVec<T> { ptr, cap, len }`. Считаем `T` ненулевого размера.
//! Реализуй:
//! - `new()` -> `Self`: пусто, без выделения (`ptr = NonNull::dangling()`, `cap = 0`,
//!   `len = 0`).
//! - `len`, `is_empty`, `capacity` - геттеры.
//! - приватный `grow(&mut self)` - как в упр. 05 (0 -> 4, иначе удвоение).
//! - `push(&mut self, value)`: если `len == cap` - `grow()`; записать `value` в
//!   `ptr.add(len)` через `.write(value)` (ячейка не инициализирована - только `write`,
//!   не присваивание); увеличить `len`.
//! - `pop(&mut self)` -> `Option<T>`: если пусто - `None`; иначе уменьшить `len` и
//!   вернуть `Some(ptr.add(len).read())` (`read` забирает значение, передавая владение
//!   наружу; повторно эту ячейку читать нельзя, но `len` уже уменьшен - инвариант цел).
//! - `get(i)` / `get_mut(i)` -> `Option<&T>` / `Option<&mut T>`: с проверкой `i < len`.
//! - `Deref<Target=[T]>` / `DerefMut`: вернуть срез `slice::from_raw_parts(ptr, len)` /
//!   `from_raw_parts_mut`. Через это заработают индексация, итерация, `.iter()`, `len()`
//!   среза и прочее - не нужно дублировать их вручную.
//! - `Drop`: уронить все `len` элементов, затем освободить буфер (если `cap != 0`).
//!   Простой способ уронить элементы - крутить `while self.pop().is_some() {}`.
//!
//! Порядок в `Drop` важен: сначала деструкторы элементов, потом `dealloc` их памяти.
//!
//! `slice::from_raw_parts`: <https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html>.
//! Разбор той же конструкции - глава "Implementing Vec" в Rustonomicon:
//! <https://doc.rust-lang.org/nomicon/vec/vec.html>.

use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn capacity(&self) -> usize {
        todo!()
    }

    fn grow(&mut self) {
        todo!()
    }

    pub fn push(&mut self, value: T) {
        let _ = value;
        todo!()
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        let _ = i;
        todo!()
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        let _ = i;
        todo!()
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
        todo!()
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        todo!()
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        todo!()
    }
}
