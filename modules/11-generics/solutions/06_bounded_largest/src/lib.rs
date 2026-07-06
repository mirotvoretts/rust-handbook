//! 06 (1x) - Минимальные ограничения. Эталонное решение.

/// Ссылка на наибольший элемент (None для пустого среза).
/// При нескольких равных максимумах - первый из них.
pub fn largest<T: PartialOrd>(xs: &[T]) -> Option<&T> {
    let mut best = xs.first()?;
    for x in &xs[1..] {
        if x > best {
            best = x;
        }
    }
    Some(best)
}

/// Ссылка на наименьший элемент (None для пустого среза).
pub fn smallest<T: PartialOrd>(xs: &[T]) -> Option<&T> {
    let mut best = xs.first()?;
    for x in &xs[1..] {
        if x < best {
            best = x;
        }
    }
    Some(best)
}

/// Отсортирован ли срез по неубыванию? (пустой и одноэлементный - да)
pub fn is_sorted<T: PartialOrd>(xs: &[T]) -> bool {
    for i in 1..xs.len() {
        if xs[i - 1] > xs[i] {
            return false;
        }
    }
    true
}
