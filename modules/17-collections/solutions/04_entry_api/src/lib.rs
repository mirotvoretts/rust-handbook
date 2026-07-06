//! 04 (1x) — Entry API. Эталонное решение.

use std::collections::HashMap;

/// Частота слов (разделение по пробелам, регистр как есть).
pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

/// Группировка слов по первой букве (слова в порядке появления).
pub fn group_by_first_char(words: &[&str]) -> HashMap<char, Vec<String>> {
    let mut groups: HashMap<char, Vec<String>> = HashMap::new();
    for word in words {
        let Some(first) = word.chars().next() else {
            continue;
        };
        groups.entry(first).or_default().push(word.to_string());
    }
    groups
}
