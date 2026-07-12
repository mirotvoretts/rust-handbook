use ex_30_01_my_vec::my_vec;

#[test]
fn empty() {
    let v: Vec<i32> = my_vec![];
    assert!(v.is_empty());
}

#[test]
fn several() {
    let v = my_vec![1, 2, 3];
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn trailing_comma() {
    let v = my_vec![10, 20, 30,];
    assert_eq!(v, vec![10, 20, 30]);
}

#[test]
fn expressions_evaluated() {
    let v = my_vec![1 + 1, 2 * 2, 9 - 1];
    assert_eq!(v, vec![2, 4, 8]);
}
