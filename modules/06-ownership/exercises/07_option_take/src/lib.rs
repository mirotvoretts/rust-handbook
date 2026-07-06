//! 07 (1x) — Перемещение значения из-под `&mut`.
//!
//! Нельзя написать `let x = *r;`, если за `&mut`-ссылкой лежит владеющее значение: это
//! оставило бы `*r` в moved-from состоянии, а компилятор требует, чтобы там всегда было
//! что-то валидное. Правильные инструменты: `Option::take`, `std::mem::replace`, `mem::take`.
//!
//! Реализуйте функции, НЕ используя `.clone()`.

use std::mem;

/// Забирает значение из `slot`, оставляя на его месте `None`. Возвращает то, что было внутри.
/// (Это ровно `Option::take`.)
pub fn take_out(slot: &mut Option<String>) -> Option<String> {
    todo!()
}

/// Кладёт `new` в `slot`, а прежнее значение возвращает. (Это ровно `mem::replace`.)
pub fn replace_with(slot: &mut String, new: String) -> String {
    todo!()
}

/// Забирает вектор из-под ссылки, оставляя на месте пустой `Vec`. (Это ровно `mem::take`.)
pub fn drain_vec(slot: &mut Vec<i32>) -> Vec<i32> {
    todo!()
}
