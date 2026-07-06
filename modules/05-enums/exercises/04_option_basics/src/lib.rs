//! 04 (0x) - Создание и разбор `Option`.
//!
//! `Option<T>` - это `Some(значение)` или `None`. Возвращайте `None` там, где значения нет,
//! и разбирайте `Option` через `match`.

/// Квадратный корень, определённый только для неотрицательных: `None` для x < 0.
pub fn safe_sqrt(x: f64) -> Option<f64> {
    todo!()
}

/// Первый элемент среза как `Option` (`None` для пустого).
pub fn first(xs: &[i32]) -> Option<i32> {
    todo!()
}

/// Текстовое описание через `match`: Some(n) -> "got N", None -> "nothing".
pub fn describe(opt: Option<i32>) -> String {
    todo!("match opt: Some(n) => format!, None => ...")
}
