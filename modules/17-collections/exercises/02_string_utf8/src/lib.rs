//! 02 (0x) — Байты против символов.
//!
//! len() — байты; chars().count() — codepoint'ы. Для кириллицы они различаются
//! вдвое. shout работает для любого языка (to_uppercase знает Unicode).

/// (длина в байтах, длина в символах).
pub fn measure(s: &str) -> (usize, usize) {
    todo!()
}

/// Первые n СИМВОЛОВ строки (не байт!) как String.
pub fn take_chars(s: &str, n: usize) -> String {
    todo!("chars().take(n).collect()")
}

/// Верхний регистр (Unicode-aware).
pub fn shout(s: &str) -> String {
    todo!()
}
