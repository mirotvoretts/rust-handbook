use sol_06_05_clone_vs_move::{first_word_len, keep_both};

#[test]
fn keeps_both_independent() {
    let (a, b) = keep_both(String::from("data"));
    assert_eq!(a, "data");
    assert_eq!(b, "data");
}

#[test]
fn first_word_len_moves_string_back() {
    let (len, s) = first_word_len(String::from("hello world"));
    assert_eq!(len, 5);
    assert_eq!(s, "hello world");

    let (len, s) = first_word_len(String::new());
    assert_eq!(len, 0);
    assert_eq!(s, "");
}
