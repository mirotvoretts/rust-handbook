use sol_03_10_recursion::{fib, pow, sum_digits};

#[test]
fn fibonacci() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
    assert_eq!(fib(2), 1);
    assert_eq!(fib(10), 55);
    assert_eq!(fib(20), 6765);
}

#[test]
fn powers() {
    assert_eq!(pow(2, 10), 1024);
    assert_eq!(pow(5, 0), 1);
    assert_eq!(pow(3, 3), 27);
    assert_eq!(pow(1, 100), 1);
}

#[test]
fn digit_sums() {
    assert_eq!(sum_digits(0), 0);
    assert_eq!(sum_digits(5), 5);
    assert_eq!(sum_digits(123), 6);
    assert_eq!(sum_digits(9999), 36);
}
