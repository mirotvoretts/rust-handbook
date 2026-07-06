//! 03 (0x) — Сигнатура с `'a` уже дана. Эталонное решение.

pub fn longer_slice<'a>(a: &'a [i32], b: &'a [i32]) -> &'a [i32] {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}
