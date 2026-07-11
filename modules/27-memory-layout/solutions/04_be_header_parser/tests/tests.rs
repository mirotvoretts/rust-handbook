use sol_27_04_be_header_parser::*;

#[test]
fn parses_exact() {
    let bytes = [0x00, 0x00, 0x01, 0x02, 0x07, 0x00, 0x10];
    assert_eq!(
        parse(&bytes),
        Some(Frame {
            magic: 0x0000_0102,
            version: 7,
            length: 0x0010,
        })
    );
}

#[test]
fn ignores_trailing_payload() {
    let bytes = [0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x00, 0x03, 0xAA, 0xBB, 0xCC];
    assert_eq!(
        parse(&bytes),
        Some(Frame {
            magic: 0xDEAD_BEEF,
            version: 1,
            length: 3,
        })
    );
}

#[test]
fn too_short_is_none() {
    assert_eq!(parse(&[]), None);
    assert_eq!(parse(&[0, 0, 0, 0, 0, 0]), None); // 6 байт, нужно 7
}
