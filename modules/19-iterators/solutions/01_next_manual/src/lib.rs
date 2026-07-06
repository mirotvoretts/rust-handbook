//! 01 (0x) - Ручной вызов next. Эталонное решение.

/// Первые два элемента через два next.
pub fn first_two(v: &[i32]) -> (i32, i32) {
    let mut it = v.iter();
    (*it.next().unwrap(), *it.next().unwrap())
}

/// Ручной подсчёт до первого None.
pub fn count_manually(v: &[i32]) -> usize {
    let mut it = v.iter();
    let mut n = 0;
    while it.next().is_some() {
        n += 1;
    }
    n
}

/// Исчерпать итератор, затем убедиться, что next даёт None.
pub fn drain_then_peek(v: &[i32]) -> (usize, Option<i32>) {
    let mut it = v.iter();
    let mut n = 0;
    while it.next().is_some() {
        n += 1;
    }
    (n, it.next().copied())
}
