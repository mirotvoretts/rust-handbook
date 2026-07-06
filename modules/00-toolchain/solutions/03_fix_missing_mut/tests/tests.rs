use sol_00_03_fix_missing_mut::countdown_from;

#[test]
fn counts_down() {
    assert_eq!(countdown_from(10), 8);
    assert_eq!(countdown_from(2), 0);
    assert_eq!(countdown_from(0), -2);
}
