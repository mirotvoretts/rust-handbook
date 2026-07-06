//! 03 (0x) - Изменяемые ссылки `&mut T`. Эталонное решение.

pub fn increment(n: &mut i32) {
    *n += 1;
}

pub fn double(n: &mut i32) {
    *n *= 2;
}

pub fn set_to(n: &mut i32, value: i32) {
    *n = value;
}
