//! 01 (0x) — Элизия: аннотации не нужны. Эталонное решение.

pub fn head(s: &str) -> &str {
    match s.chars().next() {
        Some(c) => &s[..c.len_utf8()],
        None => s,
    }
}

pub fn without_head(s: &str) -> &str {
    match s.chars().next() {
        Some(c) => &s[c.len_utf8()..],
        None => s,
    }
}
