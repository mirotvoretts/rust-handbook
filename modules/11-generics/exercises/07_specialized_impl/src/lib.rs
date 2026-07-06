//! 07 (2x) — Общий impl + impl для конкретного типа.
//!
//! У `Point<T>` два impl-блока: `impl<T>` (методы для ЛЮБОГО T) и `impl Point<f64>`
//! (геометрия — только для f64). Убедитесь, что понимаете: `Point::new(1, 2)` не имеет
//! метода `length` — он существует лишь у инстанциации с f64.

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Конструктор для любого T.
    pub fn new(x: T, y: T) -> Self {
        todo!()
    }

    /// Разбирает точку в кортеж.
    pub fn into_tuple(self) -> (T, T) {
        todo!()
    }
}

impl Point<f64> {
    /// Расстояние от начала координат: sqrt(x² + y²).
    pub fn length(&self) -> f64 {
        todo!("f64 имеет метод .sqrt()")
    }

    /// Середина отрезка между self и other.
    pub fn midpoint(&self, other: &Point<f64>) -> Point<f64> {
        todo!("среднее по каждой координате")
    }
}
