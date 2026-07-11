//! 03 (1x) - смещения полей `#[repr(C)]`-структуры (разделы 2-3 README).
//!
//! Структура `Header` уже объявлена с `#[repr(C)]`, поэтому её layout фиксирован:
//! поля идут в порядке объявления, между ними компилятор вставляет padding по правилам
//! выравнивания. Не считай смещения "в уме" - измерь их макросом `offset_of!`, а размер
//! и выравнивание - через `size_of`/`align_of`. Менять объявление `Header` нельзя.
//!
//! - `field_offsets` -> смещения полей в порядке `[tag, id, flags, count]`;
//! - `header_size` -> размер `Header`;
//! - `header_align` -> выравнивание `Header`.
//!
//! Конструкции за пределами теории:
//! - смещение поля структуры: std::mem::offset_of!
//!   https://doc.rust-lang.org/std/mem/macro.offset_of.html

#[repr(C)]
pub struct Header {
    pub tag: u8,
    pub id: u32,
    pub flags: u16,
    pub count: u8,
}

/// Смещения полей `[tag, id, flags, count]` от начала структуры.
pub fn field_offsets() -> [usize; 4] {
    todo!()
}

/// Размер структуры `Header`.
pub fn header_size() -> usize {
    todo!()
}

/// Выравнивание структуры `Header`.
pub fn header_align() -> usize {
    todo!()
}
