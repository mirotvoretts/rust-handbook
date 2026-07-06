//! 01 (0x) - Первый trait object.
//!
//! Разнотипные фигуры в одном векторе: Vec<Box<dyn Shape>>. Реализуйте Shape для
//! Circle и Rect, затем total_area, обходящую вектор через vtable-вызовы.

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub r: f64,
}

pub struct Rect {
    pub w: f64,
    pub h: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!("std::f64::consts::PI * r * r")
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        todo!()
    }
}

/// Суммарная площадь всех фигур.
pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    todo!("обычный цикл; вызов area идёт через vtable")
}
