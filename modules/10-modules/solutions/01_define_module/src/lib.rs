//! 01 (0x) — Объявление модуля с публичными функциями. Эталонное решение.

pub mod arithmetic {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn mul(a: i32, b: i32) -> i32 {
        a * b
    }
}
