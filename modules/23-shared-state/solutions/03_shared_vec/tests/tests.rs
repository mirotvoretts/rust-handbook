use sol_23_03_shared_vec::parallel_squares;

#[test]
fn squares_sorted() {
    assert_eq!(parallel_squares(vec![1, 2, 3, 4]), vec![1, 4, 9, 16]);
}

#[test]
fn duplicates() {
    assert_eq!(parallel_squares(vec![0, 1, 2, 0, 1, 2]), vec![0, 0, 1, 1, 4, 4]);
}

#[test]
fn larger() {
    let input: Vec<i64> = (1..=50).collect();
    let expected: Vec<i64> = (1..=50).map(|x| x * x).collect();
    assert_eq!(parallel_squares(input), expected);
}
