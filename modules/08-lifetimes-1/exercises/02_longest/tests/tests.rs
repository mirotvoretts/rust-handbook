use ex_08_02_longest::longest;

#[test]
fn longer_wins() {
    assert_eq!(longest("hello", "hi"), "hello");
    assert_eq!(longest("a", "bb"), "bb");
}

#[test]
fn tie_returns_first() {
    assert_eq!(longest("xx", "yy"), "xx");
}
