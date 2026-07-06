use sol_01_11_never_type::{checked_div, get_or_panic, parse_bit};

#[test]
fn bits() {
    assert_eq!(parse_bit('0'), 0);
    assert_eq!(parse_bit('1'), 1);
}

#[test]
#[should_panic]
fn bad_bit_panics() {
    parse_bit('x');
}

#[test]
fn unwrap_some() {
    assert_eq!(get_or_panic(Some(42)), 42);
}

#[test]
#[should_panic]
fn none_panics() {
    get_or_panic(None);
}

#[test]
fn division() {
    assert_eq!(checked_div(10, 2), 5);
    assert_eq!(checked_div(7, 3), 2);
}

#[test]
#[should_panic]
fn div_zero_panics() {
    checked_div(1, 0);
}
