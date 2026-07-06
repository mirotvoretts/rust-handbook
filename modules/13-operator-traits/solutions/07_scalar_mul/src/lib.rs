//! 07 (2x) — Rhs ≠ Self и унарный минус. Эталонное решение.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, k: f64) -> Vec2 {
        Vec2 { x: self.x * k, y: self.y * k }
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 { x: -self.x, y: -self.y }
    }
}

/// Линейная интерполяция: a + (b - a) * t покомпонентно.
pub fn lerp(a: Vec2, b: Vec2, t: f64) -> Vec2 {
    Vec2 {
        x: a.x + (b.x - a.x) * t,
        y: a.y + (b.y - a.y) * t,
    }
}
