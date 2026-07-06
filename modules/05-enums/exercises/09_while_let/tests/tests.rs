use ex_05_09_while_let::{pop_all, sum_drain};

#[test]
fn popping() {
    let mut s = vec![1, 2, 3];
    assert_eq!(pop_all(&mut s), vec![3, 2, 1]);
    assert!(s.is_empty());

    let mut empty: Vec<i32> = Vec::new();
    assert_eq!(pop_all(&mut empty), Vec::<i32>::new());
}

#[test]
fn draining_sum() {
    let mut s = vec![10, 20, 30];
    assert_eq!(sum_drain(&mut s), 60);
    assert!(s.is_empty());

    let mut empty: Vec<i32> = Vec::new();
    assert_eq!(sum_drain(&mut empty), 0);
}
