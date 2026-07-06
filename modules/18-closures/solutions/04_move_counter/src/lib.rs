//! 04 (1x) - Фабрики замыканий: move + impl FnMut. Эталонное решение.

/// Счётчик от 1.
pub fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

/// Аккумулятор от start.
pub fn make_adder(start: i64) -> impl FnMut(i64) -> i64 {
    let mut total = start;
    move |x| {
        total += x;
        total
    }
}
