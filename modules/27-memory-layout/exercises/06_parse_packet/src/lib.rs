//! 06 (2x) - разбор пакета со смешанным порядком байт (раздел 6 README).
//!
//! Разбери бинарный пакет по спецификации (обрати внимание: поля идут в РАЗНОМ порядке
//! байт - так бывает в реальных форматах):
//!
//!   смещение 0: `magic`       - `u16`, big-endian; обязан быть `0xCAFE`
//!   смещение 2: `version`     - `u8`
//!   смещение 3: `flags`       - `u8`
//!   смещение 4: `payload_len` - `u16`, little-endian
//!   смещение 6: `payload`     - ровно `payload_len` байт
//!
//! Реализуй `parse`, возвращающую `Result<Packet, ParseError>`:
//! - меньше 6 байт (не хватает на заголовок) -> `Err(ParseError::TooShort)`;
//! - `magic != 0xCAFE` -> `Err(ParseError::BadMagic)`;
//! - длина среза не равна `6 + payload_len` (payload обрезан или есть лишние байты) ->
//!   `Err(ParseError::LengthMismatch)`;
//! - иначе -> `Ok(Packet)`, где `payload` - копия соответствующих байт.
//!
//! Проверяй условия в этом порядке (сначала длина заголовка, затем magic, затем длина
//! payload). Кусок среза приводи к массиву через `TryInto` (M13).

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

/// Разобрать пакет по спецификации выше.
pub fn parse(bytes: &[u8]) -> Result<Packet, ParseError> {
    let _ = bytes;
    todo!()
}
