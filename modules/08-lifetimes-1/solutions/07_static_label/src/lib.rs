//! 07 (1x) — Возврат `&'static str`. Эталонное решение.

pub fn size_name(n: u32) -> &'static str {
    match n {
        0..=9 => "small",
        10..=99 => "medium",
        _ => "large",
    }
}
