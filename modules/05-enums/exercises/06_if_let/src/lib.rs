//! 06 (1x) — `if let` для одного варианта.
//!
//! Когда интересен только вариант `Some`, полный `match` избыточен — используйте `if let`.

/// Метка значения: Some(n) -> "value: N", None -> "none".
pub fn label(opt: Option<i32>) -> String {
    todo!("if let Some(n) = opt ... else ...")
}

/// Истина, если внутри `Some` лежит чётное число; для `None` — ложь.
pub fn is_some_even(opt: Option<i32>) -> bool {
    todo!()
}
