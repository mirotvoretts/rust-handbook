//! 05 (2x) — fn-указатель `fn(&str) -> usize` (неявный `for<'a>`). Эталонное решение.

pub fn byte_len(s: &str) -> usize {
    s.len()
}

pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

pub fn apply(s: &str, f: fn(&str) -> usize) -> usize {
    f(s)
}
