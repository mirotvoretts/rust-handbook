//! 05 (1x) — Обмен и изменение через `&mut`. Эталонное решение.

pub fn swap(a: &mut i32, b: &mut i32) {
    let t = *a;
    *a = *b;
    *b = t;
}

pub fn add_assign(target: &mut i32, delta: i32) {
    *target += delta;
}
