//! 11 (2x) - Обмен владением без клонов. Эталонное решение.

use std::mem;

pub fn swap_two(a: &mut String, b: &mut String) {
    mem::swap(a, b);
}

/// (a, b, c) -> (old c, old a, old b).
pub fn rotate_left3<T>(a: &mut T, b: &mut T, c: &mut T) {
    mem::swap(a, b); // (b, a, c)
    mem::swap(a, c); // (c, a, b)
}
