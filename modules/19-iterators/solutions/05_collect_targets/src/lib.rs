//! 05 (2x) - collect в разные типы + collect-в-Result. Эталонное решение.

use std::collections::HashMap;
use std::num::ParseIntError;

/// Символы в верхнем регистре в String.
pub fn shout(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_uppercase()).collect()
}

/// Индекс слово -> длина в HashMap.
pub fn word_lengths(words: &[&str]) -> HashMap<String, usize> {
    words.iter().map(|w| (w.to_string(), w.len())).collect()
}

/// Разбор всех строк; collect в Result с коротким замыканием.
pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    items.iter().map(|s| s.parse::<i32>()).collect()
}
