use sol_01_12_collatz_expr::{collatz_len, collatz_max};

#[test]
fn maxima() {
    assert_eq!(collatz_max(1), 1);
    assert_eq!(collatz_max(2), 2);
    assert_eq!(collatz_max(6), 16);
    assert_eq!(collatz_max(7), 52);
    assert_eq!(collatz_max(27), 9232);
}

#[test]
fn lengths() {
    assert_eq!(collatz_len(1), 1);
    assert_eq!(collatz_len(6), 9);
    assert_eq!(collatz_len(7), 17);
    assert_eq!(collatz_len(27), 112);
}
