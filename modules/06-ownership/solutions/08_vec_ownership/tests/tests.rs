use sol_06_08_vec_ownership::{doubled, join_owned, sum_borrowed, sum_owned};

#[test]
fn sum_owned_consumes() {
    assert_eq!(sum_owned(vec![1, 2, 3, 4]), 10);
    assert_eq!(sum_owned(Vec::new()), 0);
}

#[test]
fn sum_borrowed_keeps_vec() {
    let v = vec![10, 20, 30];
    assert_eq!(sum_borrowed(&v), 60);
    // вектор всё ещё доступен — sum_borrowed только одолжил его
    assert_eq!(v.len(), 3);
}

#[test]
fn doubled_maps() {
    let v = vec![1, 2, 3];
    assert_eq!(doubled(&v), vec![2, 4, 6]);
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn join_joins() {
    let v = vec![String::from("a"), String::from("b"), String::from("c")];
    assert_eq!(join_owned(v, "-"), "a-b-c");
}
