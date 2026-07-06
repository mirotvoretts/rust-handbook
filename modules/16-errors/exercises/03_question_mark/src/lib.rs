//! 03 (0x) — Оператор ?.
//!
//! parse_and_add написана лесенкой match — перепишите ТЕЛО через `?` (сигнатура
//! неизменна). Затем допишите mean_of_two. Оба типа ошибок здесь совпадают, поэтому
//! From-конверсия не нужна — чистый проброс.

use std::num::ParseIntError;

/// Сумма двух строк-чисел. ПЕРЕПИШИТЕ через ? — тело должно уместиться в 1-2 строки.
pub fn parse_and_add(a: &str, b: &str) -> Result<i64, ParseIntError> {
    match a.trim().parse::<i64>() {
        Ok(x) => match b.trim().parse::<i64>() {
            Ok(y) => Ok(x + y),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

/// Среднее двух строк-чисел (целочисленное деление).
pub fn mean_of_two(a: &str, b: &str) -> Result<i64, ParseIntError> {
    todo!("используйте parse_and_add? или два parse? — как нравится")
}
