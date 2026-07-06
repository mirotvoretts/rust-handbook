//! 03 (1x) - Базовые пайплайны: map / filter / collect.
//!
//! Каждую функцию решай ОДНОЙ цепочкой итераторов, без ручных циклов и push.

/// Квадраты всех элементов, в том же порядке.
pub fn squares(v: &[i32]) -> Vec<i32> {
    todo!("v.iter().map(|x| x * x).collect()")
}

/// Только чётные, значениями (не ссылками).
pub fn evens(v: &[i32]) -> Vec<i32> {
    todo!("filter + copied/cloned, затем collect")
}

/// Длины слов, у которых длина строго больше min_len.
/// Сначала отфильтруй по длине, потом отобрази в длину.
pub fn long_word_lengths(words: &[&str], min_len: usize) -> Vec<usize> {
    todo!("filter(|w| w.len() > min_len).map(|w| w.len()).collect()")
}
