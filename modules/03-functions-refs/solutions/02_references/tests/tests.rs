use sol_03_02_references::{deref_sum, len_of, max_ref};

#[test]
fn sums() {
    assert_eq!(deref_sum(&2, &3), 5);
    assert_eq!(deref_sum(&-1, &1), 0);
}

#[test]
fn maxima() {
    assert_eq!(max_ref(&7, &4), 7);
    assert_eq!(max_ref(&1, &9), 9);
    assert_eq!(max_ref(&5, &5), 5);
}

#[test]
fn borrow_keeps_ownership() {
    let s = String::from("привет");
    let n = len_of(&s);
    assert_eq!(n, 12);
    assert_eq!(s, "привет");
}
