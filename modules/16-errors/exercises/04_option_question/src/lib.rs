//! 04 (1x) - ? на Option и мост в Result.
//!
//! В функции -> Option оператор ? пробрасывает None. Смешивать миры нельзя: в
//! функции -> Result для Option нужен ok_or. Обе функции - про это.

/// Третий элемент среза, удвоенный. Всё через ?.
pub fn third_doubled(xs: &[i32]) -> Option<i32> {
    todo!("xs.get(2)? - и удвоить")
}

/// Первое слово строки заглавными. None входа или пустая строка -> Err с текстом.
pub fn first_word_upper(s: Option<&str>) -> Result<String, String> {
    todo!("s.ok_or(...)? затем split_whitespace().next().ok_or(...)?")
}
