//! 04 (0x) — Ассоциированные функции (`Type::new`).
//!
//! Функция в `impl` без `self` вызывается через имя типа: `Rectangle::new(3, 4)`.
//! Это Rust-аналог статических методов/конструкторов. `Self` внутри `impl` == `Rectangle`.

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Конструктор с заданными шириной и высотой.
    pub fn new(width: u32, height: u32) -> Self {
        todo!()
    }

    /// Конструктор квадрата со стороной `size`.
    pub fn square(size: u32) -> Self {
        todo!()
    }
}
