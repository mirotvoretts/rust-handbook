//! 02 (0x) — Видимость: сделайте элементы `pub`. Эталонное решение.

pub mod geometry {
    pub fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    pub fn perimeter(width: u32, height: u32) -> u32 {
        2 * (width + height)
    }
}
