//! 09 (2x) - Cow: аллокация по необходимости. Эталонное решение.

use std::borrow::Cow;

/// Заменяет пробелы на '_'. Без пробелов -> Borrowed.
pub fn snake(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))
    } else {
        Cow::Borrowed(s)
    }
}

/// Гарантирует префикс "id-": уже есть -> Borrowed, нет -> Owned с префиксом.
pub fn ensure_prefix(s: &str) -> Cow<'_, str> {
    if s.starts_with("id-") {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!("id-{s}"))
    }
}
