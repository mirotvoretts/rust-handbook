//! 05 (1x) - Ре-экспорт через `pub use` (фасад).
//!
//! Функция `compute` спрятана во внутреннем модуле `internal`. Сделайте её частью ПУБЛИЧНОГО
//! API крейта на верхнем уровне, добавив `pub use internal::compute;`. Тогда снаружи она
//! доступна как `crate::compute`, а не `crate::internal::compute` - деталь реализации скрыта.

mod internal {
    /// Прибавляет 100.
    pub fn compute(n: i32) -> i32 {
        todo!()
    }
}

// TODO: ре-экспортируйте: pub use internal::compute;
