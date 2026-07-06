use sol_01_07_for_range::{factorial, sum_multiples_below};

#[test]
fn factorials() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(10), 3_628_800);
}

#[test]
fn euler_one() {
    assert_eq!(sum_multiples_below(10), 23);
    assert_eq!(sum_multiples_below(1), 0);
    assert_eq!(sum_multiples_below(16), 60);
    assert_eq!(sum_multiples_below(1000), 233_168);
}
