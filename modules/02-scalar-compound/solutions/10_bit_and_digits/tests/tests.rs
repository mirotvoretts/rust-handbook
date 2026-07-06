use sol_02_10_bit_and_digits::{count_set_bits, digit_sum, is_power_of_two};

#[test]
fn popcount() {
    assert_eq!(count_set_bits(0), 0);
    assert_eq!(count_set_bits(7), 3);
    assert_eq!(count_set_bits(255), 8);
    assert_eq!(count_set_bits(1024), 1);
}

#[test]
fn digit_sums() {
    assert_eq!(digit_sum(0), 0);
    assert_eq!(digit_sum(5), 5);
    assert_eq!(digit_sum(123), 6);
    assert_eq!(digit_sum(9999), 36);
}

#[test]
fn powers_of_two() {
    assert!(is_power_of_two(1));
    assert!(is_power_of_two(2));
    assert!(is_power_of_two(1024));
    assert!(!is_power_of_two(0));
    assert!(!is_power_of_two(3));
    assert!(!is_power_of_two(1000));
}
