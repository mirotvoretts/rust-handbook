//! 04 (1x) - ? на Option и мост в Result. Эталонное решение.

/// Третий элемент среза, удвоенный. Всё через ?.
pub fn third_doubled(xs: &[i32]) -> Option<i32> {
    Some(xs.get(2)? * 2)
}

/// Первое слово строки заглавными. None входа или пустая строка -> Err с текстом.
pub fn first_word_upper(s: Option<&str>) -> Result<String, String> {
    let s = s.ok_or(String::from("нет строки"))?;
    let word = s
        .split_whitespace()
        .next()
        .ok_or(String::from("пустая строка"))?;
    Ok(word.to_uppercase())
}
