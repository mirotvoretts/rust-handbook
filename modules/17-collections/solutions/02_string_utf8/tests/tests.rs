use sol_17_02_string_utf8::{measure, shout, take_chars};

#[test]
fn ascii_vs_cyrillic() {
    assert_eq!(measure("abc"), (3, 3));
    assert_eq!(measure("мир"), (6, 3)); // кириллица: 2 байта на букву
    assert_eq!(measure(""), (0, 0));
}

#[test]
fn take_respects_chars() {
    assert_eq!(take_chars("привет", 3), "при"); // по байтам это было бы "пр" с половинкой
    assert_eq!(take_chars("ab", 10), "ab");
}

#[test]
fn shouts_unicode() {
    assert_eq!(shout("привет"), "ПРИВЕТ");
    assert_eq!(shout("hello"), "HELLO");
}
