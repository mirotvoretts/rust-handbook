//! 01 (0x) - размер, выравнивание и padding. Эталонное решение.

use std::mem::{align_of, size_of};

/// Размер и выравнивание `u64`.
pub fn u64_layout() -> (usize, usize) {
    (size_of::<u64>(), align_of::<u64>())
}

/// Размер и выравнивание кортежа `(u8, u32)`.
pub fn pair_layout() -> (usize, usize) {
    (size_of::<(u8, u32)>(), align_of::<(u8, u32)>())
}

/// Сколько байт padding в кортеже `(u8, u32)`.
pub fn pair_padding() -> usize {
    size_of::<(u8, u32)>() - (size_of::<u8>() + size_of::<u32>())
}

/// Размер и выравнивание массива `[u16; 5]`.
pub fn array_layout() -> (usize, usize) {
    (size_of::<[u16; 5]>(), align_of::<[u16; 5]>())
}
