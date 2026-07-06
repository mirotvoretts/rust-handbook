//! 10 (2x) — 2D-вектор: методы, возвращающие новые значения.
//!
//! `Vector2` — Copy-структура из двух `f64`. Методы `&self` вычисляют новые значения, не меняя
//! исходный вектор.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    /// Покомпонентная сумма двух векторов.
    pub fn add(&self, other: &Vector2) -> Vector2 {
        todo!()
    }

    /// Умножение на скаляр `k`.
    pub fn scale(&self, k: f64) -> Vector2 {
        todo!()
    }

    /// Скалярное произведение: x1*x2 + y1*y2.
    pub fn dot(&self, other: &Vector2) -> f64 {
        todo!()
    }

    /// Длина (евклидова норма): sqrt(x² + y²).
    pub fn length(&self) -> f64 {
        todo!()
    }
}
