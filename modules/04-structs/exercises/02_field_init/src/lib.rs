//! 02 (0x) - Сокращённая инициализация и обновление через `..`.
//!
//! Field init shorthand: если переменная называется как поле, пишут просто её имя.
//! Struct update syntax: `Color { поле: значение, ..base }` берёт остальные поля из `base`.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Создаёт цвет из трёх компонент, используя СОКРАЩЁННУЮ инициализацию полей.
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    todo!("Color с полями-сокращениями r, g, b")
}

/// Оттенок серого: все три компоненты равны `v`.
pub fn grey(v: u8) -> Color {
    todo!()
}

/// Возвращает копию `base` с изменённой красной компонентой (используйте `..base`).
pub fn with_red(base: Color, r: u8) -> Color {
    todo!("Color с новым r и ..base")
}
