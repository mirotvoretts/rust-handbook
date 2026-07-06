//! 05 (1x) - Doc-тесты. Эталонное решение.

/// Возводит число в квадрат.
///
/// # Примеры
/// ```
/// use sol_00_05_doctest::square;
/// assert_eq!(square(4), 16);
/// assert_eq!(square(-3), 9);
/// ```
pub fn square(n: i32) -> i32 {
    n * n
}
