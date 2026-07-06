//! 03 (0x) - Оператор ?. Эталонное решение.

use std::num::ParseIntError;

/// Сумма двух строк-чисел.
pub fn parse_and_add(a: &str, b: &str) -> Result<i64, ParseIntError> {
    Ok(a.trim().parse::<i64>()? + b.trim().parse::<i64>()?)
}

/// Среднее двух строк-чисел (целочисленное деление).
pub fn mean_of_two(a: &str, b: &str) -> Result<i64, ParseIntError> {
    Ok(parse_and_add(a, b)? / 2)
}
