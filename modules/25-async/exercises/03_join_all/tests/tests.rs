use ex_25_03_join_all::squares_concurrent;

#[test]
fn basic() {
    assert_eq!(squares_concurrent(vec![1, 2, 3, 4]), vec![1, 4, 9, 16]);
}

#[test]
fn empty() {
    assert_eq!(squares_concurrent(vec![]), Vec::<i64>::new());
}

#[test]
fn negatives() {
    assert_eq!(squares_concurrent(vec![-3, -1, 2]), vec![9, 1, 4]);
}

#[test]
fn preserves_order() {
    let input: Vec<i64> = (1..=50).collect();
    let expected: Vec<i64> = (1..=50).map(|x| x * x).collect();
    assert_eq!(squares_concurrent(input), expected);
}
