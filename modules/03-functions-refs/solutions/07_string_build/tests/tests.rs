use sol_03_07_string_build::{initials, join_with, repeat_str};

#[test]
fn repeating() {
    assert_eq!(repeat_str("ab", 3), "ababab");
    assert_eq!(repeat_str("x", 0), "");
    assert_eq!(repeat_str("", 5), "");
}

#[test]
fn joining() {
    assert_eq!(join_with("a", "b", "-"), "a-b");
    assert_eq!(join_with("left", "right", ", "), "left, right");
}

#[test]
fn inits() {
    assert_eq!(initials("john", "doe"), "JD");
    assert_eq!(initials("Alice", "Smith"), "AS");
}
