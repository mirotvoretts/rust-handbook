//! 11 (2x) — Классификация символов и шифр Цезаря. Эталонное решение.

pub fn count_alpha_digit(s: &str) -> (usize, usize) {
    let mut alpha = 0;
    let mut digit = 0;
    for c in s.chars() {
        if c.is_alphabetic() {
            alpha += 1;
        } else if c.is_numeric() {
            digit += 1;
        }
    }
    (alpha, digit)
}

pub fn caesar_shift(c: char, k: u8) -> char {
    let k = k % 26;
    if c.is_ascii_lowercase() {
        let base = b'a';
        (((c as u8 - base + k) % 26) + base) as char
    } else if c.is_ascii_uppercase() {
        let base = b'A';
        (((c as u8 - base + k) % 26) + base) as char
    } else {
        c
    }
}
