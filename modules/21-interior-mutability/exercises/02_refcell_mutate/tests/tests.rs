use ex_21_02_refcell_mutate::{double_all, len, push};
use std::cell::RefCell;

#[test]
fn push_through_shared_ref() {
    let v = RefCell::new(vec![1, 2]);
    push(&v, 3);
    push(&v, 4);
    assert_eq!(len(&v), 4);
    assert_eq!(*v.borrow(), vec![1, 2, 3, 4]);
}

#[test]
fn double_all_mutates_in_place() {
    let v = RefCell::new(vec![1, 2, 3]);
    double_all(&v);
    assert_eq!(*v.borrow(), vec![2, 4, 6]);
}

#[test]
fn len_of_empty() {
    let v: RefCell<Vec<i32>> = RefCell::new(vec![]);
    assert_eq!(len(&v), 0);
}
