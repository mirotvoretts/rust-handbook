use ex_09_05_fn_pointer::{apply, byte_len, word_count};

#[test]
fn direct() {
    assert_eq!(byte_len("hello"), 5);
    assert_eq!(word_count("a b c"), 3);
}

#[test]
fn via_apply() {
    // одна и та же apply работает с разными fn-указателями
    assert_eq!(apply("hello", byte_len), 5);
    assert_eq!(apply("привет", byte_len), 12);
    assert_eq!(apply("one two three", word_count), 3);
    assert_eq!(apply("", word_count), 0);
}
