use ex_26_01_deref_raw::read_via_raw;

#[test]
fn positive() {
    assert_eq!(read_via_raw(42), 42);
}

#[test]
fn zero() {
    assert_eq!(read_via_raw(0), 0);
}

#[test]
fn negative() {
    assert_eq!(read_via_raw(-7), -7);
}
