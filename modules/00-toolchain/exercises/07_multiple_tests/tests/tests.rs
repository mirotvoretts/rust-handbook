use ex_00_07_multiple_tests::{add, max_of, sub};

#[test]
fn addition() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
}

#[test]
fn subtraction() {
    assert_eq!(sub(5, 3), 2);
    assert_eq!(sub(0, 4), -4);
}

#[test]
fn maximum() {
    assert_eq!(max_of(3, 7), 7);
    assert_eq!(max_of(9, 2), 9);
    assert_eq!(max_of(4, 4), 4);
}
