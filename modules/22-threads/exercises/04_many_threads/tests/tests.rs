use ex_22_04_many_threads::squares;

#[test]
fn squares_in_order() {
    assert_eq!(squares(vec![1, 2, 3, 4]), vec![1, 4, 9, 16]);
}

#[test]
fn empty_input() {
    assert_eq!(squares(vec![]), Vec::<i64>::new());
}

#[test]
fn preserves_order_with_negatives() {
    assert_eq!(squares(vec![-3, 5, -1]), vec![9, 25, 1]);
}
