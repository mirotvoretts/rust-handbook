//! 04 (0x) - Сокращение пути через `use`.
//!
//! Модуль `util` содержит два помощника. Добавьте импорт `use util::{double, triple};` на
//! уровне крейта, чтобы вызывать их коротко, и реализуйте `combine` = double(n) + triple(n).

mod util {
    pub fn double(n: i32) -> i32 {
        n * 2
    }

    pub fn triple(n: i32) -> i32 {
        n * 3
    }
}

// TODO: добавьте здесь `use util::{double, triple};`

/// Возвращает double(n) + triple(n), то есть 5 * n.
pub fn combine(n: i32) -> i32 {
    todo!("double(n) + triple(n)")
}
