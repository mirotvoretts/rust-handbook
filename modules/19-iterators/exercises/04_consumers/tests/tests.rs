use ex_19_04_consumers::{
    all_positive, first_square_over, has_negative, index_of, longest, min_max, sum_of_squares,
};

#[test]
fn sums_squares() {
    assert_eq!(sum_of_squares(&[1, 2, 3]), 14);
    assert_eq!(sum_of_squares(&[]), 0);
}

#[test]
fn folds_min_max() {
    assert_eq!(min_max(&[3, 1, 4, 1, 5, 9, 2]), Some((1, 9)));
    assert_eq!(min_max(&[42]), Some((42, 42)));
    assert_eq!(min_max(&[]), None);
}

#[test]
fn finds_first_over() {
    assert_eq!(first_square_over(&[1, 2, 3, 4, 5], 10), Some(4)); // 16 > 10
    assert_eq!(first_square_over(&[1, 2], 100), None);
}

#[test]
fn any_all_predicates() {
    assert!(has_negative(&[1, -2, 3]));
    assert!(!has_negative(&[1, 2, 3]));
    assert!(all_positive(&[1, 2, 3]));
    assert!(!all_positive(&[1, 0, 3]));
    assert!(all_positive(&[])); // пустой - все подходят
}

#[test]
fn longest_first_on_tie() {
    assert_eq!(longest(&["a", "bbb", "cc", "ddd"]), Some("bbb")); // bbb и ddd оба 3, берём первое
    assert_eq!(longest(&["solo"]), Some("solo"));
    assert_eq!(longest(&[]), None);
}

#[test]
fn position_of_word() {
    assert_eq!(index_of(&["a", "b", "c"], "b"), Some(1));
    assert_eq!(index_of(&["a", "b", "c"], "z"), None);
}
