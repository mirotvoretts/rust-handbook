use ex_10_04_use_import::combine;

#[test]
fn combines() {
    assert_eq!(combine(2), 10);
    assert_eq!(combine(0), 0);
    assert_eq!(combine(-1), -5);
}
