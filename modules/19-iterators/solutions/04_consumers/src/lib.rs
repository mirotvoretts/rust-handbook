//! 04 (1x) - Потребители. Эталонное решение.

/// Сумма квадратов.
pub fn sum_of_squares(v: &[i64]) -> i64 {
    v.iter().map(|x| x * x).sum()
}

/// Мин и макс за один проход через fold.
pub fn min_max(v: &[i64]) -> Option<(i64, i64)> {
    v.iter().fold(None, |acc, &x| match acc {
        None => Some((x, x)),
        Some((lo, hi)) => Some((lo.min(x), hi.max(x))),
    })
}

/// Первое число, чей квадрат больше threshold.
pub fn first_square_over(v: &[i64], threshold: i64) -> Option<i64> {
    v.iter().copied().find(|x| x * x > threshold)
}

/// Есть ли отрицательное.
pub fn has_negative(v: &[i64]) -> bool {
    v.iter().any(|x| *x < 0)
}

/// Все ли положительны.
pub fn all_positive(v: &[i64]) -> bool {
    v.iter().all(|x| *x > 0)
}

/// Самое длинное слово, первое при равенстве длин.
pub fn longest<'a>(words: &[&'a str]) -> Option<&'a str> {
    words.iter().copied().fold(None, |best: Option<&str>, w| match best {
        Some(b) if b.len() >= w.len() => Some(b),
        _ => Some(w),
    })
}

/// Индекс первого совпадения.
pub fn index_of(words: &[&str], target: &str) -> Option<usize> {
    words.iter().position(|w| *w == target)
}
