//! 11 (2x) — Структуры из структур (композиция).
//!
//! `Line` состоит из двух `Point`. Композиция — основной способ строить сложные типы в Rust
//! (наследования нет). Методы `Line` обращаются к полям вложенных точек.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    /// Длина отрезка: расстояние между началом и концом.
    pub fn length(&self) -> f64 {
        todo!("sqrt((dx)² + (dy)²) по полям start и end")
    }

    /// Середина отрезка: точка со средними координатами.
    pub fn midpoint(&self) -> Point {
        todo!()
    }
}
