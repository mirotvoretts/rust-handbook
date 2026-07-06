use sol_00_05_doctest::square;

#[test]
fn squares() {
    assert_eq!(square(0), 0);
    assert_eq!(square(5), 25);
    assert_eq!(square(-6), 36);
}
