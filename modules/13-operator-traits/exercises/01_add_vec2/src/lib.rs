//! 01 (0x) — Перегрузка + и - для 2D-вектора.
//!
//! `a + b` — сахар для `Add::add(a, b)`. Реализуйте покомпонентные Add и Sub.
//! Vec2 — Copy, поэтому «поглощение операндов по значению» здесь бесплатно.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        todo!("покомпонентно")
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        todo!()
    }
}
