//! 06 (1x) - Видимость `pub(crate)`.
//!
//! `internal_default` помечена `pub(crate)`: она видна везде ВНУТРИ крейта, но не снаружи -
//! тесты (отдельный крейт) вызвать её напрямую не могут. Зато её использует публичная
//! `effective`. Реализуйте `effective`: вернуть переданное значение, а при `None` - значение
//! по умолчанию из `internal_default`.

pub mod config {
    pub(crate) fn internal_default() -> i32 {
        7
    }

    /// Возвращает `override_val`, если он задан, иначе значение по умолчанию.
    pub fn effective(override_val: Option<i32>) -> i32 {
        todo!("override_val.unwrap_or(internal_default())")
    }
}
