use ex_08_09_longest_word::longest_word;

#[test]
fn finds_longest() {
    assert_eq!(longest_word(&["a", "bbb", "cc"]), Some("bbb"));
    assert_eq!(longest_word(&["rust", "is", "great"]), Some("great"));
}

#[test]
fn tie_returns_first() {
    assert_eq!(longest_word(&["aa", "bb", "c"]), Some("aa"));
}

#[test]
fn empty() {
    assert_eq!(longest_word(&[]), None);
}
