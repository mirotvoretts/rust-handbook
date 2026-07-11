//! 04 (1x) - разбор фиксированного big-endian заголовка. Эталонное решение.

#[derive(Debug, PartialEq, Eq)]
pub struct Frame {
    pub magic: u32,
    pub version: u8,
    pub length: u16,
}

/// Разобрать 7-байтовый big-endian заголовок из начала `bytes`.
pub fn parse(bytes: &[u8]) -> Option<Frame> {
    if bytes.len() < 7 {
        return None;
    }
    let magic = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
    let version = bytes[4];
    let length = u16::from_be_bytes(bytes[5..7].try_into().unwrap());
    Some(Frame {
        magic,
        version,
        length,
    })
}
