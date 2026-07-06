use ex_07_03_borrow_scope::{push_len, push_sum};

#[test]
fn sums() {
    let mut v = vec![1, 2, 3];
    push_sum(&mut v);
    assert_eq!(v, vec![1, 2, 3, 6]);

    let mut empty: Vec<i32> = Vec::new();
    push_sum(&mut empty);
    assert_eq!(empty, vec![0]);
}

#[test]
fn lens() {
    let mut v = vec![5, 5];
    push_len(&mut v);
    assert_eq!(v, vec![5, 5, 2]);
}
