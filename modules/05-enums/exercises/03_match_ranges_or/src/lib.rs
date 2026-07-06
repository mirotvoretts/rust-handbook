//! 03 (0x) - Диапазоны, or-паттерны и guard в `match`.
//!
//! Паттерны богаче, чем просто константы: диапазон `90..=100`, альтернативы `a | e | i`,
//! и условие-guard `паттерн if условие`.

/// Буквенная оценка через диапазоны: 90..=100 'A', 80..=89 'B', 70..=79 'C', 60..=69 'D', иначе 'F'.
pub fn grade(score: u32) -> char {
    todo!("match со сравнением диапазонов")
}

/// Гласная ли (латинская, строчная) - через or-паттерн в `matches!` или `match`.
pub fn is_vowel(c: char) -> bool {
    todo!("matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')")
}

/// Классификация числа через guard: "negative" / "zero" / "positive".
pub fn classify(n: i32) -> &'static str {
    todo!("ветки: 0 => zero, n if n < 0 => negative, _ => positive")
}
