use sol_16_01_option_combinators::{first_char, len_or_zero, require};

#[test]
fn len_or_zero_works() {
    assert_eq!(len_or_zero(Some("hello")), 5);
    assert_eq!(len_or_zero(None), 0);
}

#[test]
fn first_char_flattens() {
    assert_eq!(first_char(Some("abc")), Some('a'));
    assert_eq!(first_char(Some("")), None); // Some пустой строки -> None
    assert_eq!(first_char(None), None);
}

#[test]
fn require_converts() {
    assert_eq!(require(Some(5)), Ok(5));
    assert!(require(None).is_err());
}
