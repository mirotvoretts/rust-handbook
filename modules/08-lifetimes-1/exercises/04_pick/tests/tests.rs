use ex_08_04_pick::pick;

#[test]
fn picks() {
    assert_eq!(pick("a", "b", true), "a");
    assert_eq!(pick("a", "b", false), "b");
}
