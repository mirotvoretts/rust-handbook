use ex_18_01_closure_syntax::{greet, square, sum_via_capture};

#[test]
fn square_works() {
    assert_eq!(square(5), 25);
    assert_eq!(square(-4), 16);
    assert_eq!(square(0), 0);
}

#[test]
fn sum_captures_both() {
    assert_eq!(sum_via_capture(2, 3), 5);
    assert_eq!(sum_via_capture(-10, 4), -6);
}

#[test]
fn greet_reads_name() {
    assert_eq!(greet("Rust"), "Hello, Rust!");
    assert_eq!(greet(""), "Hello, !");
}
