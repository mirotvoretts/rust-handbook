use ex_27_06_parse_packet::*;

#[test]
fn valid_packet() {
    // magic 0xCAFE (BE), version 3, flags 1, payload_len 2 (LE), payload [0xAA, 0xBB]
    let bytes = [0xCA, 0xFE, 0x03, 0x01, 0x02, 0x00, 0xAA, 0xBB];
    assert_eq!(
        parse(&bytes),
        Ok(Packet {
            version: 3,
            flags: 1,
            payload: vec![0xAA, 0xBB],
        })
    );
}

#[test]
fn empty_payload() {
    let bytes = [0xCA, 0xFE, 0x00, 0x00, 0x00, 0x00];
    assert_eq!(
        parse(&bytes),
        Ok(Packet {
            version: 0,
            flags: 0,
            payload: vec![],
        })
    );
}

#[test]
fn too_short() {
    assert_eq!(parse(&[0xCA, 0xFE, 0x00]), Err(ParseError::TooShort));
}

#[test]
fn bad_magic() {
    let bytes = [0xDE, 0xAD, 0x00, 0x00, 0x00, 0x00];
    assert_eq!(parse(&bytes), Err(ParseError::BadMagic));
}

#[test]
fn payload_truncated() {
    // payload_len = 4, а байт payload только 2
    let bytes = [0xCA, 0xFE, 0x00, 0x00, 0x04, 0x00, 0xAA, 0xBB];
    assert_eq!(parse(&bytes), Err(ParseError::LengthMismatch));
}

#[test]
fn trailing_bytes() {
    // payload_len = 1, а байт payload 2 - лишний байт
    let bytes = [0xCA, 0xFE, 0x00, 0x00, 0x01, 0x00, 0xAA, 0xBB];
    assert_eq!(parse(&bytes), Err(ParseError::LengthMismatch));
}
