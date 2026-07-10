use ex_24_02_channel_collect::collect_squares;

#[test]
fn preserves_order() {
    assert_eq!(collect_squares(vec![1, 2, 3, 4]), vec![1, 4, 9, 16]);
}

#[test]
fn empty() {
    assert_eq!(collect_squares(vec![]), Vec::<i64>::new());
}

#[test]
fn order_matters() {
    assert_eq!(collect_squares(vec![3, 1, 2]), vec![9, 1, 4]);
}
