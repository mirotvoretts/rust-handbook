//! 06 (2x) - Ленивые адаптеры. Эталонное решение.

/// (индекс, символ).
pub fn indexed_chars(s: &str) -> Vec<(usize, char)> {
    s.chars().enumerate().collect()
}

/// Поэлементная сумма, стоп по короткому.
pub fn pairwise_sum(a: &[i32], b: &[i32]) -> Vec<i32> {
    a.iter().zip(b).map(|(x, y)| x + y).collect()
}

/// Обратный порядок.
pub fn reversed(v: &[i32]) -> Vec<i32> {
    v.iter().rev().copied().collect()
}

/// Брать, пока меньше limit.
pub fn take_below(v: &[i32], limit: i32) -> Vec<i32> {
    v.iter().take_while(|&&x| x < limit).copied().collect()
}

/// Плоский вектор из среза векторов.
pub fn flatten_rows(rows: &[Vec<i32>]) -> Vec<i32> {
    rows.iter().flat_map(|r| r.iter().copied()).collect()
}

/// Префиксные суммы через scan.
pub fn running_sum(v: &[i64]) -> Vec<i64> {
    v.iter()
        .scan(0i64, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect()
}
