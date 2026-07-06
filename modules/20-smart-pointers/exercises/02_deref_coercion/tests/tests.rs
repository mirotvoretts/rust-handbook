use ex_20_02_deref_coercion::{boxed_len, first_char, greet_boxed};

#[test]
fn len_through_box() {
    assert_eq!(boxed_len(Box::new(String::from("hello"))), 5);
    assert_eq!(boxed_len(Box::new(String::new())), 0);
}

#[test]
fn coercion_into_str_arg() {
    assert_eq!(greet_boxed(Box::new(String::from("Rust"))), "Hello, Rust!");
}

#[test]
fn first_char_through_box() {
    assert_eq!(first_char(Box::new(String::from("abc"))), Some('a'));
    assert_eq!(first_char(Box::new(String::new())), None);
}
