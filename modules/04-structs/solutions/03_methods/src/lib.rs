//! 03 (0x) — Методы `&self`. Эталонное решение.

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}
