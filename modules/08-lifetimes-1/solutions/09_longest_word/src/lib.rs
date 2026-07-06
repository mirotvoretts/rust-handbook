//! 09 (2x) — Самое длинное слово из среза строк. Эталонное решение.

pub fn longest_word<'a>(words: &[&'a str]) -> Option<&'a str> {
    let mut best: Option<&'a str> = None;
    for &w in words {
        match best {
            Some(b) if b.len() >= w.len() => {}
            _ => best = Some(w),
        }
    }
    best
}
