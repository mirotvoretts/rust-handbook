//! 08 (1x) - Строковые срезы `&str`. Эталонное решение.

pub fn byte_len(s: &str) -> usize {
    s.len()
}

pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn starts_upper(s: &str) -> bool {
    match s.chars().next() {
        Some(c) => c.is_uppercase(),
        None => false,
    }
}
