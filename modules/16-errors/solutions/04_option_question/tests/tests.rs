use sol_16_04_option_question::{first_word_upper, third_doubled};

#[test]
fn third() {
    assert_eq!(third_doubled(&[1, 2, 3, 4]), Some(6));
    assert_eq!(third_doubled(&[1, 2]), None);
}

#[test]
fn first_word() {
    assert_eq!(first_word_upper(Some("hello world")), Ok(String::from("HELLO")));
    assert!(first_word_upper(Some("   ")).is_err());
    assert!(first_word_upper(None).is_err());
}
