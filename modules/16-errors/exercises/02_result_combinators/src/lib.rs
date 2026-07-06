//! 02 (0x) - Комбинаторы Result.
//!
//! map преобразует Ok, map_err - Err, ok выбрасывает ошибку в Option,
//! unwrap_or_else вычисляет запасное значение ИЗ ошибки.

/// Удваивает число внутри Ok.
pub fn double_ok(r: Result<i32, String>) -> Result<i32, String> {
    todo!("map")
}

/// Оборачивает текст ошибки в "error: ...".
pub fn tag_error(r: Result<i32, String>) -> Result<i32, String> {
    todo!("map_err + format!")
}

/// Result -> Option, ошибка отбрасывается.
pub fn to_option(r: Result<i32, String>) -> Option<i32> {
    todo!()
}

/// Значение или длина текста ошибки как запасной результат.
pub fn value_or_err_len(r: Result<i32, String>) -> i32 {
    todo!("unwrap_or_else(|e| ...)")
}
