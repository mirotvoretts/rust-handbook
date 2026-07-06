use ex_16_05_custom_error::{validate_nickname, NicknameError};

#[test]
fn valid_names_pass_through() {
    assert_eq!(validate_nickname("alice_42".into()), Ok(String::from("alice_42")));
    assert_eq!(validate_nickname("Bob".into()), Ok(String::from("Bob")));
}

#[test]
fn length_checks() {
    assert_eq!(validate_nickname("ab".into()), Err(NicknameError::TooShort { len: 2 }));
    assert_eq!(
        validate_nickname("abcdefghijklm".into()),
        Err(NicknameError::TooLong { len: 13 })
    );
}

#[test]
fn char_checks() {
    assert_eq!(validate_nickname("1abc".into()), Err(NicknameError::BadStart { ch: '1' }));
    assert_eq!(validate_nickname("ab-c".into()), Err(NicknameError::BadChar { ch: '-' }));
}

#[test]
fn display_is_implemented() {
    let e = NicknameError::TooShort { len: 1 };
    assert!(!e.to_string().is_empty()); // текст любой, но Display обязан работать
}
