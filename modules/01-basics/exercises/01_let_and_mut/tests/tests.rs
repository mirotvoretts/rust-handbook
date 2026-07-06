use ex_01_01_let_and_mut::{increment_thrice, running_total, swapped};

#[test]
fn increments_by_three() {
    assert_eq!(increment_thrice(0), 3);
    assert_eq!(increment_thrice(10), 13);
    assert_eq!(increment_thrice(-5), -2);
}

#[test]
fn accumulates_total() {
    assert_eq!(running_total(1, 2, 3), 6);
    assert_eq!(running_total(-1, 5, 0), 4);
}

#[test]
fn swaps_pair() {
    assert_eq!(swapped(1, 2), (2, 1));
    assert_eq!(swapped(0, -7), (-7, 0));
}
