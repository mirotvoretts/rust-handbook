use sol_01_10_loop_state::{int_sqrt, reverse_digits};

#[test]
fn integer_sqrt() {
    assert_eq!(int_sqrt(0), 0);
    assert_eq!(int_sqrt(1), 1);
    assert_eq!(int_sqrt(15), 3);
    assert_eq!(int_sqrt(16), 4);
    assert_eq!(int_sqrt(24), 4);
    assert_eq!(int_sqrt(25), 5);
    assert_eq!(int_sqrt(1_000_000), 1000);
}

#[test]
fn reversed_digits() {
    assert_eq!(reverse_digits(123), 321);
    assert_eq!(reverse_digits(100), 1);
    assert_eq!(reverse_digits(0), 0);
    assert_eq!(reverse_digits(57), 75);
}
