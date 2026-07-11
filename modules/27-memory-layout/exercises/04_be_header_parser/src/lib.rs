//! 04 (1x) - разбор фиксированного big-endian заголовка (раздел 6 README).
//!
//! Во многих бинарных форматах в начале идёт заголовок фиксированной длины с полями в
//! network byte order (big-endian). Разбери такой заголовок из `&[u8]`:
//!
//!   смещение 0: `magic`   - `u32`, big-endian
//!   смещение 4: `version` - `u8`
//!   смещение 5: `length`  - `u16`, big-endian
//!
//! Итого 7 байт. Реализуй `parse`:
//! - если во входе меньше 7 байт - верни `None`;
//! - иначе прочитай поля (порядок байт - `from_be_bytes`) и верни `Some(Frame)`;
//! - лишние байты после 7-го игнорируй (это уже payload).
//!
//! Кусок среза приводи к массиву фиксированной длины через `TryInto` (M13):
//! `let a: [u8; 4] = bytes[0..4].try_into().unwrap();` - длину ты уже проверил.

#[derive(Debug, PartialEq, Eq)]
pub struct Frame {
    pub magic: u32,
    pub version: u8,
    pub length: u16,
}

/// Разобрать 7-байтовый big-endian заголовок из начала `bytes`.
pub fn parse(bytes: &[u8]) -> Option<Frame> {
    let _ = bytes;
    todo!()
}
