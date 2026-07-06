//! 09 (2x) — Cow: аллокация по необходимости.
//!
//! Правило: если строка уже «хорошая» — верните Cow::Borrowed (ноль аллокаций);
//! только если пришлось менять — Cow::Owned. Тесты проверяют вариант matches!.

use std::borrow::Cow;

/// Заменяет пробелы на '_'. Без пробелов -> Borrowed.
pub fn snake(s: &str) -> Cow<'_, str> {
    todo!("if s.contains(' ') ... replace ... else Borrowed")
}

/// Гарантирует префикс "id-": уже есть -> Borrowed, нет -> Owned с префиксом.
pub fn ensure_prefix(s: &str) -> Cow<'_, str> {
    todo!("starts_with")
}
