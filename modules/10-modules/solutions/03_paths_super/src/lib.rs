//! 03 (0x) — Обращение к родительскому модулю через `super::`. Эталонное решение.

pub fn base_value() -> i32 {
    10
}

pub mod child {
    pub fn plus_base(n: i32) -> i32 {
        n + super::base_value()
    }
}
