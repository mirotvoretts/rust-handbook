//! 01 (0x) - Первые обобщённые функции. Эталонное решение.

/// Возвращает аргумент как есть.
pub fn identity<T>(x: T) -> T {
    x
}

/// Ссылка на первый элемент среза (или None для пустого).
pub fn first<T>(xs: &[T]) -> Option<&T> {
    if xs.is_empty() { None } else { Some(&xs[0]) }
}

/// Ссылка на последний элемент среза (или None для пустого).
pub fn last<T>(xs: &[T]) -> Option<&T> {
    if xs.is_empty() { None } else { Some(&xs[xs.len() - 1]) }
}
