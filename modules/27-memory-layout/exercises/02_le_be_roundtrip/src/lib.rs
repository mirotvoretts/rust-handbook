//! 02 (0x) - байтовое представление `u32` и порядок байт (раздел 6 README).
//!
//! Реализуй четыре преобразования `u32` <-> `[u8; 4]`, явно выбирая порядок байт
//! методами `u32::to_le_bytes`/`to_be_bytes` и `u32::from_le_bytes`/`from_be_bytes`.
//! Своими руками раскладывать число по байтам (сдвигами) не нужно.
//!
//! - `to_le` / `to_be` - число в массив (little- и big-endian);
//! - `from_le` / `from_be` - массив обратно в число.

/// `u32` -> байты, little-endian (младший байт первым).
pub fn to_le(n: u32) -> [u8; 4] {
    let _ = n;
    todo!()
}

/// `u32` -> байты, big-endian (старший байт первым).
pub fn to_be(n: u32) -> [u8; 4] {
    let _ = n;
    todo!()
}

/// Байты little-endian -> `u32`.
pub fn from_le(bytes: [u8; 4]) -> u32 {
    let _ = bytes;
    todo!()
}

/// Байты big-endian -> `u32`.
pub fn from_be(bytes: [u8; 4]) -> u32 {
    let _ = bytes;
    todo!()
}
