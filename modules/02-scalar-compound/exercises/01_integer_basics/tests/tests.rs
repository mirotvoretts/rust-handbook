use ex_02_01_integer_basics::{i8_max, truncate_to_u8, widen};

#[test]
fn limits() {
    assert_eq!(i8_max(), 127);
}

#[test]
fn truncation() {
    assert_eq!(truncate_to_u8(65), 65);
    assert_eq!(truncate_to_u8(256), 0);
    assert_eq!(truncate_to_u8(300), 44);
    assert_eq!(truncate_to_u8(-1), 255);
}

#[test]
fn widening() {
    assert_eq!(widen(0), 0);
    assert_eq!(widen(200), 200);
    assert_eq!(widen(255), 255);
}
