//! 03 (0x) — Диапазоны, or-паттерны и guard в `match`. Эталонное решение.

pub fn grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

pub fn classify(n: i32) -> &'static str {
    match n {
        0 => "zero",
        n if n < 0 => "negative",
        _ => "positive",
    }
}
