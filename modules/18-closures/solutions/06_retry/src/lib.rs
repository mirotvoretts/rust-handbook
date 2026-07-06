//! 06 (2x) - Комбинатор retry(n, f). Эталонное решение.

/// Повторяет f до первого Ok, но не более n попыток (минимум одна).
pub fn retry<T, E, F>(n: u32, mut f: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut last = f(); // хотя бы одна попытка
    let mut left = n.saturating_sub(1);
    while last.is_err() && left > 0 {
        last = f();
        left -= 1;
    }
    last
}
