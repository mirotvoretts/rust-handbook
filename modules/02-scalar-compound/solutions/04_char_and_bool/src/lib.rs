//! 04 (0x) - `char` и `bool`. Эталонное решение.

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

pub fn digit_value(c: char) -> Option<u32> {
    c.to_digit(10)
}

pub fn xor(a: bool, b: bool) -> bool {
    a ^ b
}
