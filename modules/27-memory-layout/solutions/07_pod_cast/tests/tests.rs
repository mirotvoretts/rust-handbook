use sol_27_07_pod_cast::*;

fn sample() -> Vec<Sample> {
    vec![
        Sample { t: 1, value: 10 },
        Sample { t: 2, value: -5 },
        Sample { t: 3, value: 100 },
    ]
}

#[test]
fn encode_len() {
    // три записи по 8 байт
    assert_eq!(encode(&sample()).len(), 24);
}

#[test]
fn roundtrip() {
    let s = sample();
    assert_eq!(decode(&encode(&s)), Some(s));
}

#[test]
fn empty_roundtrip() {
    assert_eq!(encode(&[]), Vec::<u8>::new());
    assert_eq!(decode(&[]), Some(Vec::new()));
}

#[test]
fn bad_length_is_none() {
    assert_eq!(decode(&[0u8; 7]), None);
    assert_eq!(decode(&[0u8; 9]), None);
    assert_eq!(total_value(&[0u8; 7]), None);
}

#[test]
fn total() {
    assert_eq!(total_value(&encode(&sample())), Some(105));
}
