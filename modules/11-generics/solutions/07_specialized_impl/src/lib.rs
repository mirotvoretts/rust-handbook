//! 07 (2x) — Общий impl + impl для конкретного типа. Эталонное решение.

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Конструктор для любого T.
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    /// Разбирает точку в кортеж.
    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl Point<f64> {
    /// Расстояние от начала координат: sqrt(x² + y²).
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Середина отрезка между self и other.
    pub fn midpoint(&self, other: &Point<f64>) -> Point<f64> {
        Point {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }
}
