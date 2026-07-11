//! 04 (1x) - безопасная обёртка над `from_raw_parts_mut`.
//!
//! Реализуй `split_at_mut(slice, mid)` - аналог `slice::split_at_mut`: верни две
//! изменяемые непересекающиеся части исходного среза - `[0, mid)` и `[mid, len)`.
//! Наивная безопасная версия (`(&mut slice[..mid], &mut slice[mid..])`) отвергается
//! borrow checker'ом: он не доказывает, что части не пересекаются. Поэтому части
//! строят из сырого указателя, а корректность обеспечивают проверкой `mid` и знанием
//! о непересечении (раздел 4 README). Сигнатура остаётся безопасной.
//!
//! Если `mid > slice.len()`, паникуй (как это делает стандартный `split_at_mut`).
//!
//! Конструкции за пределами теории:
//! - собрать срез из указателя и длины: std::slice::from_raw_parts_mut
//!   https://doc.rust-lang.org/std/slice/fn.from_raw_parts_mut.html
//! - получить `*mut` на начало среза: <[T]>::as_mut_ptr
//!   https://doc.rust-lang.org/std/primitive.slice.html#method.as_mut_ptr

/// Раздели срез на две изменяемые части по индексу `mid`.
pub fn split_at_mut(slice: &mut [i64], mid: usize) -> (&mut [i64], &mut [i64]) {
    let _ = (slice, mid);
    todo!()
}
