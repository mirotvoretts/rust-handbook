use sol_08_11_pair_longer::Pair;

#[test]
fn longer_field() {
    assert_eq!(Pair::new("hello", "hi").longer(), "hello");
    assert_eq!(Pair::new("a", "bbb").longer(), "bbb");
}

#[test]
fn tie_returns_a() {
    assert_eq!(Pair::new("xx", "yy").longer(), "xx");
}
