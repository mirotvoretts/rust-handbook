use ex_22_05_scoped_sum::parallel_sum;

#[test]
fn sums_even_length() {
    assert_eq!(parallel_sum(&[1, 2, 3, 4]), 10);
}

#[test]
fn sums_odd_length() {
    assert_eq!(parallel_sum(&[1, 2, 3, 4, 5]), 15);
}

#[test]
fn empty_and_single() {
    assert_eq!(parallel_sum(&[]), 0);
    assert_eq!(parallel_sum(&[42]), 42);
}

#[test]
fn borrows_not_moves() {
    // после вызова срез всё ещё доступен вызывающему: его лишь одалживали
    let data = vec![10, 20, 30];
    assert_eq!(parallel_sum(&data), 60);
    assert_eq!(data.len(), 3);
}
