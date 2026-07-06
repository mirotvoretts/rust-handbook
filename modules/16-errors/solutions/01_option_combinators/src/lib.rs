//! 01 (0x) — Комбинаторы Option. Эталонное решение.

/// Длина строки внутри Option (None -> 0).
pub fn len_or_zero(s: Option<&str>) -> usize {
    s.map(|s| s.len()).unwrap_or(0)
}

/// Первый символ строки, если строка непуста.
pub fn first_char(s: Option<&str>) -> Option<char> {
    s.and_then(|s| s.chars().next())
}

/// Option -> Result с кодом ошибки.
pub fn require(v: Option<i32>) -> Result<i32, String> {
    v.ok_or(String::from("значение отсутствует"))
}
