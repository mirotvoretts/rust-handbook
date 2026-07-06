//! 07 (1x) - Построение `String` из `&str`. Эталонное решение.

pub fn repeat_str(s: &str, n: usize) -> String {
    s.repeat(n)
}

pub fn join_with(a: &str, b: &str, sep: &str) -> String {
    format!("{a}{sep}{b}")
}

pub fn initials(first: &str, last: &str) -> String {
    let f = first.chars().next().unwrap().to_ascii_uppercase();
    let l = last.chars().next().unwrap().to_ascii_uppercase();
    format!("{f}{l}")
}
