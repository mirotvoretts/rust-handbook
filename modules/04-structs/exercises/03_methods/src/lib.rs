//! 03 (0x) — Методы `&self`.
//!
//! Присоедините поведение к `Rectangle` в блоке `impl`. Методы, которые только читают поля,
//! берут `&self`. Вызов: `r.area()`.

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Площадь прямоугольника.
    pub fn area(&self) -> u32 {
        todo!("self.width * self.height")
    }

    /// Периметр прямоугольника.
    pub fn perimeter(&self) -> u32 {
        todo!()
    }

    /// Квадрат ли (ширина равна высоте).
    pub fn is_square(&self) -> bool {
        todo!()
    }
}
