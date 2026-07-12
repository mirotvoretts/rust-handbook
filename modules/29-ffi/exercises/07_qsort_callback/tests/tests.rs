use ex_29_07_qsort_callback::*;

#[test]
fn sorts_mixed() {
    let mut v = [5i32, -3, 0, 42, -100, 7];
    sort_i32(&mut v);
    assert_eq!(v, [-100, -3, 0, 5, 7, 42]);
}

#[test]
fn already_sorted() {
    let mut v = [1i32, 2, 3];
    sort_i32(&mut v);
    assert_eq!(v, [1, 2, 3]);
}

#[test]
fn duplicates() {
    let mut v = [3i32, 1, 3, 1, 2];
    sort_i32(&mut v);
    assert_eq!(v, [1, 1, 2, 3, 3]);
}

#[test]
fn empty_and_single() {
    let mut e: [i32; 0] = [];
    sort_i32(&mut e);
    assert!(e.is_empty());

    let mut one = [42i32];
    sort_i32(&mut one);
    assert_eq!(one, [42]);
}
