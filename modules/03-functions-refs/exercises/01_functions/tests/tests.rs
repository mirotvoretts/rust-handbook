use ex_03_01_functions::{abs_val, is_even, square};

#[test]
fn squares() {
    assert_eq!(square(4), 16);
    assert_eq!(square(-3), 9);
    assert_eq!(square(0), 0);
}

#[test]
fn evenness() {
    assert!(is_even(4));
    assert!(!is_even(7));
    assert!(is_even(0));
}

#[test]
fn absolutes() {
    assert_eq!(abs_val(-5), 5);
    assert_eq!(abs_val(5), 5);
    assert_eq!(abs_val(0), 0);
}
