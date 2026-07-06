use ex_02_06_arrays::{contains5, reverse5, sum5};

#[test]
fn sums() {
    assert_eq!(sum5([1, 2, 3, 4, 5]), 15);
    assert_eq!(sum5([0, 0, 0, 0, 0]), 0);
    assert_eq!(sum5([-1, 1, -2, 2, 10]), 10);
}

#[test]
fn reversal() {
    assert_eq!(reverse5([1, 2, 3, 4, 5]), [5, 4, 3, 2, 1]);
    assert_eq!(reverse5([7, 7, 7, 7, 7]), [7, 7, 7, 7, 7]);
}

#[test]
fn membership() {
    assert!(contains5([1, 2, 3, 4, 5], 3));
    assert!(contains5([1, 2, 3, 4, 5], 5));
    assert!(!contains5([1, 2, 3, 4, 5], 9));
}
