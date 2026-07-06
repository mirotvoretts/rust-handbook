use ex_02_08_str_basics::{byte_len, char_count, first_char, starts_upper};

#[test]
fn bytes_vs_chars() {
    assert_eq!(byte_len("abc"), 3);
    assert_eq!(char_count("abc"), 3);
    // кириллица: каждый символ занимает 2 байта в UTF-8
    assert_eq!(byte_len("привет"), 12);
    assert_eq!(char_count("привет"), 6);
    assert_eq!(byte_len("я"), 2);
    assert_eq!(char_count("я"), 1);
}

#[test]
fn firsts() {
    assert_eq!(first_char("hi"), Some('h'));
    assert_eq!(first_char("привет"), Some('п'));
    assert_eq!(first_char(""), None);
}

#[test]
fn capitalization() {
    assert!(starts_upper("Hello"));
    assert!(!starts_upper("hello"));
    assert!(!starts_upper(""));
}
