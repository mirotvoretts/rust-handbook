use ex_25_02_await_loop::process_all;

#[test]
fn basic() {
    assert_eq!(process_all(vec![-2, 0, 5]), vec![-4, 0, 10]);
}

#[test]
fn empty() {
    assert_eq!(process_all(vec![]), Vec::<i64>::new());
}

#[test]
fn preserves_order() {
    assert_eq!(process_all(vec![1, 2, 3, 4]), vec![2, 4, 6, 8]);
}

#[test]
fn larger() {
    let input: Vec<i64> = (1..=100).collect();
    let expected: Vec<i64> = (1..=100).map(|x| x * 2).collect();
    assert_eq!(process_all(input), expected);
}
