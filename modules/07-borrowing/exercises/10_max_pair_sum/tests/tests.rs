use ex_07_10_max_pair_sum::max_pair_sum;

#[test]
fn sums() {
    assert_eq!(max_pair_sum(&[1, 2, 3, 4]), Some(7));
    assert_eq!(max_pair_sum(&[-1, -2, -3]), Some(-3));
    assert_eq!(max_pair_sum(&[10, -5, 10]), Some(5));
}

#[test]
fn too_short() {
    assert_eq!(max_pair_sum(&[5]), None);
    assert_eq!(max_pair_sum(&[]), None);
}
