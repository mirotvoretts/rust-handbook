//! 06 (1x) — Видимость `pub(crate)`. Эталонное решение.

pub mod config {
    pub(crate) fn internal_default() -> i32 {
        7
    }

    pub fn effective(override_val: Option<i32>) -> i32 {
        override_val.unwrap_or(internal_default())
    }
}
