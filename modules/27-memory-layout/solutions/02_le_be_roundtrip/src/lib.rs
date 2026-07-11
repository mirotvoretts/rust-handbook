//! 02 (0x) - байтовое представление `u32` и порядок байт. Эталонное решение.

/// `u32` -> байты, little-endian.
pub fn to_le(n: u32) -> [u8; 4] {
    n.to_le_bytes()
}

/// `u32` -> байты, big-endian.
pub fn to_be(n: u32) -> [u8; 4] {
    n.to_be_bytes()
}

/// Байты little-endian -> `u32`.
pub fn from_le(bytes: [u8; 4]) -> u32 {
    u32::from_le_bytes(bytes)
}

/// Байты big-endian -> `u32`.
pub fn from_be(bytes: [u8; 4]) -> u32 {
    u32::from_be_bytes(bytes)
}
