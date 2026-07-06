//! 08 (1x) — `Result` с `Ok`/`Err`.
//!
//! Функция, которая может дать сбой, честно возвращает `Result<T, E>`. Ошибку кодируем
//! значением (здесь — сообщением `String`).

/// Целочисленное деление. При делении на ноль — `Err("cannot divide by zero")`.
pub fn checked_div(a: i32, b: i32) -> Result<i32, String> {
    todo!("Err(String::from(\"cannot divide by zero\")) либо Ok(a / b)")
}

/// Разбор булева значения: "true" -> Ok(true), "false" -> Ok(false),
/// иначе -> Err("invalid bool: <s>").
pub fn parse_bool(s: &str) -> Result<bool, String> {
    todo!("match s ...")
}
