use ex_03_08_call_helpers::{count_primes_below, is_prime};

#[test]
fn primality() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(9));
    assert!(is_prime(13));
    assert!(!is_prime(100));
}

#[test]
fn counting() {
    assert_eq!(count_primes_below(2), 0);
    assert_eq!(count_primes_below(10), 4); // 2,3,5,7
    assert_eq!(count_primes_below(20), 8);
}
