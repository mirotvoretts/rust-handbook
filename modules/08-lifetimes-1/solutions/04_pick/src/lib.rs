//! 04 (0x) — Выбор одной из двух ссылок по флагу. Эталонное решение.

pub fn pick<'a>(first: &'a str, second: &'a str, use_first: bool) -> &'a str {
    if use_first {
        first
    } else {
        second
    }
}
