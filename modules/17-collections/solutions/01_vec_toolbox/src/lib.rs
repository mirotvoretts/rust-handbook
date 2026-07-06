//! 01 (0x) — Инструменты Vec. Эталонное решение.

/// Убирает нечётные на месте.
pub fn keep_even(v: &mut Vec<i32>) {
    v.retain(|x| x % 2 == 0);
}

/// Дозаписывает все элементы из среза.
pub fn append_all(v: &mut Vec<String>, extra: &[&str]) {
    v.extend(extra.iter().map(|s| s.to_string()));
}

/// Схлопывает СОСЕДНИЕ дубликаты.
pub fn squash(v: &mut Vec<i32>) {
    v.dedup();
}

/// Вектор квадратов 1..=n БЕЗ реаллокаций.
pub fn squares(n: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(n);
    for i in 1..=n as u64 {
        v.push(i * i);
    }
    v
}
