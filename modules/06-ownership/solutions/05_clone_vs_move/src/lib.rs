//! 05 (1x) - Когда нужен `.clone()`, а когда хватает move. Эталонное решение.

/// Возвращает два независимых владельца - клонирование неизбежно.
pub fn keep_both(s: String) -> (String, String) {
    (s.clone(), s)
}

/// Длину считаем по ссылке, строку возвращаем через move - без клонов.
pub fn first_word_len(s: String) -> (usize, String) {
    let len = s.split(' ').next().map_or(0, |w| w.len());
    (len, s)
}
