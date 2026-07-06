//! 01 (0x) — Комбинаторы Option.
//!
//! Каждую функцию можно решить match'ем — но задача в том, чтобы обойтись ОДНОЙ
//! цепочкой комбинаторов: map / and_then / unwrap_or / ok_or.

/// Длина строки внутри Option (None -> 0).
pub fn len_or_zero(s: Option<&str>) -> usize {
    todo!("map + unwrap_or")
}

/// Первый символ строки, если строка непуста.
pub fn first_char(s: Option<&str>) -> Option<char> {
    todo!("and_then: s.chars().next() сам возвращает Option")
}

/// Option -> Result с кодом ошибки.
pub fn require(v: Option<i32>) -> Result<i32, String> {
    todo!("ok_or со String")
}
