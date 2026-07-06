//! 08 (1x) — Строковые срезы `&str`.
//!
//! Строки в Rust — UTF-8. Длина в БАЙТАХ (`.len()`) и число СИМВОЛОВ (`.chars().count()`) —
//! разные величины: для не-ASCII они не совпадают. Это ключевая интуиция про строки.

/// Длина строки в байтах (UTF-8).
pub fn byte_len(s: &str) -> usize {
    todo!("s.len()")
}

/// Число символов (Unicode scalar values). Подсказка: `s.chars().count()`.
pub fn char_count(s: &str) -> usize {
    todo!()
}

/// Первый символ строки; `None` для пустой строки. Подсказка: `s.chars().next()`.
pub fn first_char(s: &str) -> Option<char> {
    todo!()
}

/// Начинается ли строка с заглавной буквы (первый символ в верхнем регистре).
/// Для пустой строки — false.
pub fn starts_upper(s: &str) -> bool {
    todo!("возьмите первый символ и спросите is_uppercase()")
}
