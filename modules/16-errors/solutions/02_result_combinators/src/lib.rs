//! 02 (0x) — Комбинаторы Result. Эталонное решение.

/// Удваивает число внутри Ok.
pub fn double_ok(r: Result<i32, String>) -> Result<i32, String> {
    r.map(|n| n * 2)
}

/// Оборачивает текст ошибки в "error: ...".
pub fn tag_error(r: Result<i32, String>) -> Result<i32, String> {
    r.map_err(|e| format!("error: {e}"))
}

/// Result -> Option, ошибка отбрасывается.
pub fn to_option(r: Result<i32, String>) -> Option<i32> {
    r.ok()
}

/// Значение или длина текста ошибки как запасной результат.
pub fn value_or_err_len(r: Result<i32, String>) -> i32 {
    r.unwrap_or_else(|e| e.len() as i32)
}
