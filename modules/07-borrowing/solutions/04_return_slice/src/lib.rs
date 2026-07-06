//! 04 (0x) - Возврат заимствованного среза. Эталонное решение.

pub fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

pub fn tail(xs: &[i32]) -> &[i32] {
    if xs.is_empty() {
        xs
    } else {
        &xs[1..]
    }
}
