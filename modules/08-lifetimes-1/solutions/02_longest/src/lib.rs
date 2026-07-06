//! 02 (0x) — ДОБАВЬТЕ `'a` САМИ. Эталонное решение.

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}
