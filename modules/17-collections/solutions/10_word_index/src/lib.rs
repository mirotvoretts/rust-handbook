//! 10 (3x) - Капстоун: индекс слов. Эталонное решение.

use std::collections::HashMap;

pub struct WordIndex {
    /// слово (в нижнем регистре) -> позиции (порядковые номера слов, с нуля)
    positions: HashMap<String, Vec<usize>>,
}

impl WordIndex {
    /// Строит индекс текста.
    pub fn build(text: &str) -> Self {
        let mut positions: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, word) in text.split_whitespace().enumerate() {
            positions.entry(word.to_lowercase()).or_default().push(i);
        }
        WordIndex { positions }
    }

    /// Позиции слова (регистронезависимо). Пусто -> &[].
    pub fn find(&self, word: &str) -> &[usize] {
        self.positions
            .get(&word.to_lowercase())
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Топ-n слов по частоте; при равенстве - лексикографически меньшее раньше.
    pub fn top_n(&self, n: usize) -> Vec<(String, usize)> {
        let mut counts: Vec<(String, usize)> = self
            .positions
            .iter()
            .map(|(w, ps)| (w.clone(), ps.len()))
            .collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        counts.truncate(n);
        counts
    }
}
