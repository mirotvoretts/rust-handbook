use ex_02_09_slice_scan::{count_adjacent_equal, is_sorted_asc, pairwise_sums};

#[test]
fn sorted() {
    assert!(is_sorted_asc(&[]));
    assert!(is_sorted_asc(&[42]));
    assert!(is_sorted_asc(&[1, 2, 2, 3]));
    assert!(!is_sorted_asc(&[1, 3, 2]));
    assert!(!is_sorted_asc(&[3, 2, 1]));
}

#[test]
fn adjacent_equal() {
    assert_eq!(count_adjacent_equal(&[1, 1, 2, 2, 2]), 3);
    assert_eq!(count_adjacent_equal(&[1, 2, 3]), 0);
    assert_eq!(count_adjacent_equal(&[7]), 0);
    assert_eq!(count_adjacent_equal(&[]), 0);
}

#[test]
fn pairsums() {
    assert_eq!(pairwise_sums(&[1, 2, 3, 4]), vec![3, 5, 7]);
    assert_eq!(pairwise_sums(&[5]), Vec::<i32>::new());
    assert_eq!(pairwise_sums(&[]), Vec::<i32>::new());
}
