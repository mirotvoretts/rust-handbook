//! 06 (2x) - разбор пакета со смешанным порядком байт. Эталонное решение.

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    TooShort,
    BadMagic,
    LengthMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    pub version: u8,
    pub flags: u8,
    pub payload: Vec<u8>,
}

/// Разобрать пакет по спецификации.
pub fn parse(bytes: &[u8]) -> Result<Packet, ParseError> {
    if bytes.len() < 6 {
        return Err(ParseError::TooShort);
    }
    let magic = u16::from_be_bytes(bytes[0..2].try_into().unwrap());
    if magic != 0xCAFE {
        return Err(ParseError::BadMagic);
    }
    let version = bytes[2];
    let flags = bytes[3];
    let payload_len = u16::from_le_bytes(bytes[4..6].try_into().unwrap()) as usize;

    if bytes.len() != 6 + payload_len {
        return Err(ParseError::LengthMismatch);
    }
    Ok(Packet {
        version,
        flags,
        payload: bytes[6..].to_vec(),
    })
}
