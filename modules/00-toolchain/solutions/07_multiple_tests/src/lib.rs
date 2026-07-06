//! 07 (1x) — Несколько функций и несколько тестов. Эталонное решение.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn max_of(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}
