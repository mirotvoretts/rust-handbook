use ex_17_07_set_operations::{common, is_covered, only_in_first};

#[test]
fn intersection_sorted_unique() {
    assert_eq!(common(&[3, 1, 2, 3], &[2, 3, 4]), vec![2, 3]);
    assert_eq!(common(&[1], &[2]), Vec::<i32>::new());
}

#[test]
fn difference_works() {
    assert_eq!(only_in_first(&[1, 2, 3], &[2]), vec![1, 3]);
    assert_eq!(only_in_first(&[], &[1]), Vec::<i32>::new());
}

#[test]
fn subset_check() {
    assert!(is_covered(&[1, 2], &[1, 2, 3]));
    assert!(is_covered(&[], &[1]));
    assert!(!is_covered(&[1, 4], &[1, 2, 3]));
}
