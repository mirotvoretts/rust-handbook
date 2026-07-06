//! 10 (2x) — 2D-вектор: методы, возвращающие новые значения. Эталонное решение.

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn scale(&self, k: f64) -> Vector2 {
        Vector2 {
            x: self.x * k,
            y: self.y * k,
        }
    }

    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
