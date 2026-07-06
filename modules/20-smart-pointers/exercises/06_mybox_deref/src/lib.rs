//! 06 (2x) - Свой умный указатель: Deref / DerefMut.
//!
//! Реализовав Deref, ваш тип начинает вести себя как ссылка: работает *b,
//! методы T доступны напрямую, включается deref coercion. Так устроен и Box
//! (только он ещё и кладёт значение на кучу; здесь для простоты храним по значению).

use std::ops::{Deref, DerefMut};

/// Учебный умный указатель - обёртка вокруг одного значения.
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    /// Обернуть значение.
    pub fn new(x: T) -> Self {
        todo!("MyBox(x)")
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    /// Дать общую ссылку на внутреннее значение (тогда *b и методы T заработают).
    fn deref(&self) -> &T {
        todo!("&self.0")
    }
}

impl<T> DerefMut for MyBox<T> {
    /// Дать эксклюзивную ссылку (тогда через *b можно менять значение).
    fn deref_mut(&mut self) -> &mut T {
        todo!("&mut self.0")
    }
}
