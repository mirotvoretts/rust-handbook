use sol_27_08_tlv::*;

#[test]
fn empty_is_ok_empty() {
    assert_eq!(parse_all(&[]), Ok(vec![]));
}

#[test]
fn single_record() {
    // typ 7, len 2 (BE), value [0xAA, 0xBB]
    let bytes = [0x07, 0x00, 0x02, 0xAA, 0xBB];
    assert_eq!(
        parse_all(&bytes),
        Ok(vec![Tlv {
            typ: 7,
            value: vec![0xAA, 0xBB],
        }])
    );
}

#[test]
fn multiple_records() {
    let bytes = [
        0x01, 0x00, 0x01, 0xAA, // typ 1, len 1
        0x02, 0x00, 0x00, // typ 2, len 0 (пустое значение)
        0x03, 0x00, 0x03, 0xBB, 0xCC, 0xDD, // typ 3, len 3
    ];
    assert_eq!(
        parse_all(&bytes),
        Ok(vec![
            Tlv { typ: 1, value: vec![0xAA] },
            Tlv { typ: 2, value: vec![] },
            Tlv { typ: 3, value: vec![0xBB, 0xCC, 0xDD] },
        ])
    );
}

#[test]
fn truncated_header() {
    // всего 2 байта - на заголовок (3) не хватает
    assert_eq!(parse_all(&[0x01, 0x00]), Err(TlvError::Truncated));
}

#[test]
fn truncated_value() {
    // len 4, а значения только 2 байта
    let bytes = [0x01, 0x00, 0x04, 0xAA, 0xBB];
    assert_eq!(parse_all(&bytes), Err(TlvError::Truncated));
}

#[test]
fn truncated_second_record() {
    let bytes = [
        0x01, 0x00, 0x01, 0xAA, // корректная первая запись
        0x02, 0x00, // обрезанный заголовок второй
    ];
    assert_eq!(parse_all(&bytes), Err(TlvError::Truncated));
}
