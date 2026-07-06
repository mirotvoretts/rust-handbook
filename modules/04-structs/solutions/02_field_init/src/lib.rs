//! 02 (0x) - Сокращённая инициализация и обновление через `..`. Эталонное решение.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b }
}

pub fn grey(v: u8) -> Color {
    Color { r: v, g: v, b: v }
}

pub fn with_red(base: Color, r: u8) -> Color {
    Color { r, ..base }
}
