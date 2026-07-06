use sol_01_06_while_loops::{collatz_steps, gcd};

#[test]
fn euclid_gcd() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(18, 48), 6);
    assert_eq!(gcd(7, 1), 1);
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
    assert_eq!(gcd(100, 10), 10);
}

#[test]
fn collatz() {
    assert_eq!(collatz_steps(1), 0);
    assert_eq!(collatz_steps(6), 8);
    assert_eq!(collatz_steps(7), 16);
}
