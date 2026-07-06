//! 04 (1x) — Entry API.
//!
//! Правило: если пишете contains_key + insert — вы ищете ключ дважды. entry ищет
//! один раз. Обе функции — по одному entry-выражению в теле цикла.

use std::collections::HashMap;

/// Частота слов (разделение по пробелам, регистр как есть).
pub fn word_count(text: &str) -> HashMap<String, usize> {
    todo!("*map.entry(...).or_insert(0) += 1")
}

/// Группировка слов по первой букве (слова в порядке появления).
pub fn group_by_first_char(words: &[&str]) -> HashMap<char, Vec<String>> {
    todo!("entry(...).or_insert_with(Vec::new).push(...); пустые слова пропускайте")
}
