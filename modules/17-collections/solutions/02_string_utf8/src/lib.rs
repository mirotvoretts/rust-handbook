//! 02 (0x) — Байты против символов. Эталонное решение.

/// (длина в байтах, длина в символах).
pub fn measure(s: &str) -> (usize, usize) {
    (s.len(), s.chars().count())
}

/// Первые n СИМВОЛОВ строки (не байт!) как String.
pub fn take_chars(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

/// Верхний регистр (Unicode-aware).
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}
