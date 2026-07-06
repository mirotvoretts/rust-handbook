use ex_20_01_box_basics::{boxed, sum_boxed, unbox};

#[test]
fn box_roundtrip() {
    let b = boxed(42);
    assert_eq!(*b, 42);
    assert_eq!(unbox(b), 42);
}

#[test]
fn box_negative() {
    assert_eq!(unbox(boxed(-7)), -7);
}

#[test]
fn sum_two_boxes() {
    assert_eq!(sum_boxed(boxed(3), boxed(4)), 7);
    assert_eq!(sum_boxed(boxed(-10), boxed(10)), 0);
}
