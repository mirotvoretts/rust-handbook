//! 08 (3x) - вектор из типов нулевого размера (раздел 6 README).
//!
//! `MyVec` из упр. 06 в начале требует `size_of::<T>() != 0`. Причина: для ZST
//! (zero-sized type, например `()` или пустая структура) вся арифметика буфера рушится -
//! `ptr.add(len)` не двигается, а `Layout::array` даёт нулевой размер, который нельзя
//! передавать в `alloc`. Правильный ответ: под ZST память вообще НЕ нужна. Тысяча
//! значений нулевого размера занимает ноль байт; хранить достаточно одну лишь длину.
//!
//! Но есть тонкость: даже у ZST может быть `Drop`. `push` забирает значение во владение,
//! `pop`/`Drop` обязаны запустить его деструктор столько раз, сколько элементов было
//! добавлено, - иначе побочные эффекты деструктора потеряются.
//!
//! Тип объявлен: `ZstVec<T> { len, _marker: PhantomData<T> }` (`PhantomData` "владеет"
//! `T` для системы типов, не занимая места). Реализуй:
//! - `new()` -> `Self`: `len = 0`. Оставь в начале `assert_eq!(size_of::<T>(), 0, ...)` -
//!   тип рассчитан только на ZST.
//! - `len`, `is_empty`, `capacity` (ёмкость условно бесконечна: верни `usize::MAX`).
//! - `push(&mut self, value)`: памяти под `value` нет, но и уронить его сейчас нельзя -
//!   иначе деструктор сработает раньше времени. Забудь значение без вызова деструктора
//!   (`std::mem::forget(value)`) и увеличь `len`. (Позже `pop`/`Drop` "воссоздадут" по
//!   одному экземпляру и уронят их - для ZST это корректно, т.к. байт всё равно нет.)
//! - `pop(&mut self)` -> `Option<T>`: если пусто - `None`; иначе уменьшить `len` и
//!   вернуть `Some(NonNull::<T>::dangling().as_ptr().read())`. Чтение ZST из висячего
//!   указателя не читает ни одного байта, поэтому корректно и даёт владеемый `T`,
//!   деструктор которого отработает у вызывающего.
//! - `Drop`: уронить все оставшиеся `len` элементов (проще всего `while pop().is_some()`).
//!
//! `mem::forget`: <https://doc.rust-lang.org/std/mem/fn.forget.html>.
//! Почему ZST обрабатывают отдельно - тот же раздел Rustonomicon про Vec:
//! <https://doc.rust-lang.org/nomicon/vec/vec-zsts.html>.

use std::marker::PhantomData;

pub struct ZstVec<T> {
    len: usize,
    _marker: PhantomData<T>,
}

impl<T> ZstVec<T> {
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

    pub fn push(&mut self, value: T) {
        let _ = value;
        todo!()
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }
}

impl<T> Default for ZstVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for ZstVec<T> {
    fn drop(&mut self) {
        todo!()
    }
}
