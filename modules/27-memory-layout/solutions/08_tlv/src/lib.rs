//! 08 (3x) - разбор потока записей type-length-value. Эталонное решение.

#[derive(Debug, PartialEq, Eq)]
pub struct Tlv {
    pub typ: u8,
    pub value: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TlvError {
    Truncated,
}

/// Разобрать весь поток TLV-записей.
pub fn parse_all(bytes: &[u8]) -> Result<Vec<Tlv>, TlvError> {
    let mut out = Vec::new();
    let mut pos = 0;

    while pos < bytes.len() {
        // Заголовок: 1 байт typ + 2 байта len.
        if pos + 3 > bytes.len() {
            return Err(TlvError::Truncated);
        }
        let typ = bytes[pos];
        let len = u16::from_be_bytes(bytes[pos + 1..pos + 3].try_into().unwrap()) as usize;
        let value_start = pos + 3;

        // Хватает ли байт на объявленное значение.
        if value_start + len > bytes.len() {
            return Err(TlvError::Truncated);
        }
        out.push(Tlv {
            typ,
            value: bytes[value_start..value_start + len].to_vec(),
        });
        pos = value_start + len;
    }

    Ok(out)
}
