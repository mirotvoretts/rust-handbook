//! 06 (2x) - Ленивые адаптеры: enumerate / zip / rev / take_while / flat_map / scan.
//!
//! Каждую функцию решай цепочкой адаптеров + один потребитель в конце.

/// Пары (индекс, символ) для всех символов строки. Индекс с нуля.
pub fn indexed_chars(s: &str) -> Vec<(usize, char)> {
    todo!("s.chars().enumerate().collect()")
}

/// Поэлементные суммы двух срезов. Длина результата - по короткому (zip).
pub fn pairwise_sum(a: &[i32], b: &[i32]) -> Vec<i32> {
    todo!("a.iter().zip(b).map(|(x, y)| x + y).collect()")
}

/// Элементы в обратном порядке, значениями.
pub fn reversed(v: &[i32]) -> Vec<i32> {
    todo!("v.iter().rev().copied().collect()")
}

/// Ведущий блок строго возрастающих префиксных элементов? Нет - проще:
/// бери элементы, ПОКА они меньше limit; на первом >= limit остановись.
pub fn take_below(v: &[i32], limit: i32) -> Vec<i32> {
    todo!("v.iter().take_while(|&&x| x < limit).copied().collect()")
}

/// Разверни срез срезов в один плоский вектор (flat_map или flatten).
pub fn flatten_rows(rows: &[Vec<i32>]) -> Vec<i32> {
    todo!("rows.iter().flat_map(|r| r.iter().copied()).collect()")
}

/// Префиксные суммы: на позиции i - сумма v[0..=i]. Реши через scan,
/// а не ручным циклом с внешним аккумулятором.
pub fn running_sum(v: &[i64]) -> Vec<i64> {
    todo!("v.iter().scan(0i64, |acc, x| { *acc += x; Some(*acc) }).collect()")
}
