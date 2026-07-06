//! 07 (1x) — Комбинаторы `Option`.
//!
//! Вместо ручного `match` над `Option` часто используют методы: `.map`, `.unwrap_or`, `.filter`.

/// Удваивает значение внутри `Some`, `None` оставляет `None`. Используйте `.map`.
pub fn double_opt(opt: Option<i32>) -> Option<i32> {
    todo!("opt.map(|x| x * 2)")
}

/// Возвращает значение или 0, если `None`. Используйте `.unwrap_or`.
pub fn or_zero(opt: Option<i32>) -> i32 {
    todo!()
}

/// Оставляет `Some` только для положительных значений, иначе `None`. Используйте `.filter`.
pub fn keep_positive(opt: Option<i32>) -> Option<i32> {
    todo!("opt.filter(|&x| x > 0)")
}
