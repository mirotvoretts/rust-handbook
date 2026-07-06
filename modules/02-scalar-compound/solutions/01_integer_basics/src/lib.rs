//! 01 (0x) - Целые типы: границы и приведение `as`. Эталонное решение.

pub fn i8_max() -> i8 {
    i8::MAX
}

pub fn truncate_to_u8(n: i32) -> u8 {
    n as u8
}

pub fn widen(n: u8) -> u64 {
    n as u64
}
