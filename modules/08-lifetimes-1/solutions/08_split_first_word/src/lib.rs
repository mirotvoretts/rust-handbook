//! 08 (1x) - Пара срезов из одного входа. Эталонное решение.

pub fn split_first_word(s: &str) -> (&str, &str) {
    match s.find(' ') {
        Some(i) => (&s[..i], &s[i + 1..]),
        None => (s, ""),
    }
}
