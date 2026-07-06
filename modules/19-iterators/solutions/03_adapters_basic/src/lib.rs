//! 03 (1x) - Базовые пайплайны. Эталонное решение.

/// Квадраты.
pub fn squares(v: &[i32]) -> Vec<i32> {
    v.iter().map(|x| x * x).collect()
}

/// Только чётные, значениями.
pub fn evens(v: &[i32]) -> Vec<i32> {
    v.iter().filter(|x| *x % 2 == 0).copied().collect()
}

/// Длины слов длиннее min_len.
pub fn long_word_lengths(words: &[&str], min_len: usize) -> Vec<usize> {
    words
        .iter()
        .filter(|w| w.len() > min_len)
        .map(|w| w.len())
        .collect()
}
