use ex_00_08_fix_all::sum_to;

#[test]
fn sums() {
    assert_eq!(sum_to(5), 15);
    assert_eq!(sum_to(1), 1);
    assert_eq!(sum_to(0), 0);
    assert_eq!(sum_to(-3), 0);
    assert_eq!(sum_to(100), 5050);
}
