use sol_00_04_fix_typo::triple;

#[test]
fn triples() {
    assert_eq!(triple(5), 15);
    assert_eq!(triple(0), 0);
    assert_eq!(triple(-2), -6);
}
