//! 03 (1x) - смещения полей `#[repr(C)]`-структуры. Эталонное решение.

use std::mem::{align_of, offset_of, size_of};

#[repr(C)]
pub struct Header {
    pub tag: u8,
    pub id: u32,
    pub flags: u16,
    pub count: u8,
}

/// Смещения полей `[tag, id, flags, count]`.
pub fn field_offsets() -> [usize; 4] {
    [
        offset_of!(Header, tag),
        offset_of!(Header, id),
        offset_of!(Header, flags),
        offset_of!(Header, count),
    ]
}

/// Размер структуры `Header`.
pub fn header_size() -> usize {
    size_of::<Header>()
}

/// Выравнивание структуры `Header`.
pub fn header_align() -> usize {
    align_of::<Header>()
}
