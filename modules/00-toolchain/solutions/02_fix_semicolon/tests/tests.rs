use sol_00_02_fix_semicolon::add_one;

#[test]
fn increments() {
    assert_eq!(add_one(4), 5);
    assert_eq!(add_one(-1), 0);
    assert_eq!(add_one(0), 1);
}
