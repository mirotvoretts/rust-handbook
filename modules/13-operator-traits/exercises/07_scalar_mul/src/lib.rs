//! 07 (2x) — Rhs ≠ Self и унарный минус.
//!
//! Вектор умножается на СКАЛЯР: `impl Mul<f64> for Vec2` — параметр Rhs здесь не Self.
//! Плюс унарный Neg: `-v`. Заметьте, `v * 2.0` работает, а `2.0 * v` — нет (для этого
//! нужен был бы impl Mul<Vec2> for f64 — тоже законно, но не в этом упражнении).

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, k: f64) -> Vec2 {
        todo!()
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        todo!()
    }
}

/// Линейная интерполяция: a + (b - a) * t, но БЕЗ Add/Sub — выразите покомпонентно
/// или через уже реализованные Mul/Neg.
pub fn lerp(a: Vec2, b: Vec2, t: f64) -> Vec2 {
    todo!("x: a.x + (b.x - a.x) * t, аналогично y")
}
