use ex_22_03_spawn_sum::spawn_sum;

#[test]
fn sums_vector() {
    assert_eq!(spawn_sum(vec![1, 2, 3, 4]), 10);
}

#[test]
fn empty_is_zero() {
    assert_eq!(spawn_sum(vec![]), 0);
}

#[test]
fn does_not_overflow_i32_range() {
    // сумма выходит за i32, но помещается в i64
    assert_eq!(spawn_sum(vec![i32::MAX, i32::MAX]), 2 * i32::MAX as i64);
}
