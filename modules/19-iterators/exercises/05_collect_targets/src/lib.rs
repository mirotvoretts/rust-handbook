//! 05 (2x) - collect в разные типы + collect-в-Result.
//!
//! Целевой тип collect выбирается по аннотации/турбофишу. Отдельный приём -
//! собрать итератор из Result в один Result, с коротким замыканием на первой Err.

use std::collections::HashMap;
use std::num::ParseIntError;

/// Собери символы в String, приведя каждый к верхнему регистру.
/// char::to_ascii_uppercase возвращает char.
pub fn shout(s: &str) -> String {
    todo!("s.chars().map(|c| c.to_ascii_uppercase()).collect()")
}

/// Построй индекс слово -> длина. При повторах слов достаточно любой длины
/// (они одинаковы). Собери пары в HashMap одним collect.
pub fn word_lengths(words: &[&str]) -> HashMap<String, usize> {
    todo!("words.iter().map(|w| (w.to_string(), w.len())).collect()")
}

/// Разбери ВСЕ строки в числа. Если хоть одна не парсится - верни первую ошибку
/// (Err), не собирая остаток. Тип цели - Result<Vec<i32>, ParseIntError>.
pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    todo!("items.iter().map(|s| s.parse::<i32>()).collect()")
}
