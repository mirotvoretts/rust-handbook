//! 02 (0x) — Явные операции при переполнении. Эталонное решение.

pub fn wrapping_add_u8(a: u8, b: u8) -> u8 {
    a.wrapping_add(b)
}

pub fn checked_add_u8(a: u8, b: u8) -> Option<u8> {
    a.checked_add(b)
}

pub fn saturating_sub_u8(a: u8, b: u8) -> u8 {
    a.saturating_sub(b)
}
