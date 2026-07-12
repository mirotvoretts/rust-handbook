//! 02 (0x) - `#[repr(C)] union` и type punning. Эталонное решение.

#[repr(C)]
pub union Bits32 {
    pub u: u32,
    pub f: f32,
    pub bytes: [u8; 4],
}

/// `Bits32` с записанным полем `u`.
pub fn from_u32(x: u32) -> Bits32 {
    Bits32 { u: x }
}

/// `Bits32` с записанным полем `f`.
pub fn from_f32(x: f32) -> Bits32 {
    Bits32 { f: x }
}

/// Байты `b` как `u32`.
pub fn as_u32(b: &Bits32) -> u32 {
    // SAFETY: любой набор из 4 байт валиден для u32.
    unsafe { b.u }
}

/// Байты `b` как `f32`.
pub fn as_f32(b: &Bits32) -> f32 {
    // SAFETY: любой набор из 4 байт валиден для f32 (включая NaN).
    unsafe { b.f }
}

/// Байты `b` в порядке этой платформы.
pub fn as_ne_bytes(b: &Bits32) -> [u8; 4] {
    // SAFETY: любой набор из 4 байт валиден для [u8; 4].
    unsafe { b.bytes }
}
