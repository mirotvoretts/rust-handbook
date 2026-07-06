use sol_08_08_split_first_word::split_first_word;

#[test]
fn splits() {
    assert_eq!(split_first_word("hello world"), ("hello", "world"));
    assert_eq!(split_first_word("a b c"), ("a", "b c"));
    assert_eq!(split_first_word("hello"), ("hello", ""));
    assert_eq!(split_first_word(""), ("", ""));
}
