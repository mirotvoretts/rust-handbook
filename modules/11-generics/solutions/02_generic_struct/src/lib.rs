//! 02 (0x) — Обобщённая структура. Эталонное решение.

/// Пара значений одного типа.
pub struct Pair<T> {
    pub left: T,
    pub right: T,
}

/// Собирает пару из двух значений.
pub fn make_pair<T>(left: T, right: T) -> Pair<T> {
    Pair { left, right }
}

/// Разбирает пару обратно в кортеж.
pub fn into_tuple<T>(p: Pair<T>) -> (T, T) {
    (p.left, p.right)
}
