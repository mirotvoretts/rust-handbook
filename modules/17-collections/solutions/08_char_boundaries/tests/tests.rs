use sol_17_08_char_boundaries::{char_offset, truncate_bytes};

#[test]
fn ascii_truncation() {
    assert_eq!(truncate_bytes("hello", 10), "hello"); // влезает — без «…»
    assert_eq!(truncate_bytes("hello world", 5), "hello…");
}

#[test]
fn cyrillic_no_panic_mid_char() {
    // "привет" = 12 байт (по 2 на букву); лимит 5 режет посреди «и» -> берём 4 байта
    assert_eq!(truncate_bytes("привет", 5), "пр…");
    assert_eq!(truncate_bytes("привет", 6), "при…");
}

#[test]
fn offsets() {
    assert_eq!(char_offset("abc", 1), Some(1));
    assert_eq!(char_offset("мир", 1), Some(2)); // «м» занимает 2 байта
    assert_eq!(char_offset("ab", 5), None);
}
