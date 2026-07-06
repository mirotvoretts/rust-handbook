//! 06 (2x) - Свой умный указатель: Deref / DerefMut. Эталонное решение.

use std::ops::{Deref, DerefMut};

/// Обёртка вокруг одного значения.
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    /// Обернуть значение.
    pub fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
