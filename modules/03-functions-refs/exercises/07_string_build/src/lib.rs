//! 07 (1x) - Построение `String` из `&str`.
//!
//! Функции принимают заимствованные `&str` и возвращают новую владеющую `String`.
//! Удобны `str::repeat`, макрос `format!`, методы символов.

/// Повторяет строку `n` раз: ("ab", 3) -> "ababab", ("x", 0) -> "".
pub fn repeat_str(s: &str, n: usize) -> String {
    todo!("s.repeat(n)")
}

/// Склеивает `a` и `b` через разделитель `sep`: ("a","b","-") -> "a-b".
pub fn join_with(a: &str, b: &str, sep: &str) -> String {
    todo!("format!(...)")
}

/// Инициалы: первые буквы `first` и `last` в верхнем регистре, слитно.
/// ("john","doe") -> "JD". Считайте, что обе строки непусты и начинаются с ASCII-буквы.
pub fn initials(first: &str, last: &str) -> String {
    todo!()
}
