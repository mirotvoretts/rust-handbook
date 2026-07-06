//! 10 (3x) — Капстоун: индекс слов.
//!
//! Соедините модуль: HashMap + entry, Vec, сортировка с tie-break. Слова — по
//! split_whitespace, приведение к нижнему регистру.

use std::collections::HashMap;

pub struct WordIndex {
    /// слово (в нижнем регистре) -> позиции (порядковые номера слов, с нуля)
    positions: HashMap<String, Vec<usize>>,
}

impl WordIndex {
    /// Строит индекс текста.
    pub fn build(text: &str) -> Self {
        todo!("enumerate по split_whitespace, to_lowercase, entry")
    }

    /// Позиции слова (регистронезависимо). Пусто -> &[].
    pub fn find(&self, word: &str) -> &[usize] {
        todo!("map(|v| v.as_slice()).unwrap_or(&[])")
    }

    /// Топ-n слов по частоте; при равенстве — лексикографически меньшее раньше.
    pub fn top_n(&self, n: usize) -> Vec<(String, usize)> {
        todo!("соберите (слово, count) в Vec, sort_by с двумя ключами, truncate")
    }
}
