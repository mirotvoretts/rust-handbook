use ex_07_02_unique_borrow::{push_doubled, scale_all};

#[test]
fn pushing() {
    let mut v = vec![1, 2];
    push_doubled(&mut v, 5);
    assert_eq!(v, vec![1, 2, 10]);
}

#[test]
fn scaling() {
    let mut v = vec![1, 2, 3];
    scale_all(&mut v, 10);
    assert_eq!(v, vec![10, 20, 30]);

    let mut empty: Vec<i32> = Vec::new();
    scale_all(&mut empty, 2);
    assert_eq!(empty, Vec::<i32>::new());
}
