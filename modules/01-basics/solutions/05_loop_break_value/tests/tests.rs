use sol_01_05_loop_break_value::{first_pow2_at_least, sum_1_to};

#[test]
fn powers_of_two() {
    assert_eq!(first_pow2_at_least(1), 1);
    assert_eq!(first_pow2_at_least(5), 8);
    assert_eq!(first_pow2_at_least(8), 8);
    assert_eq!(first_pow2_at_least(100), 128);
}

#[test]
fn triangular_sums() {
    assert_eq!(sum_1_to(0), 0);
    assert_eq!(sum_1_to(1), 1);
    assert_eq!(sum_1_to(5), 15);
    assert_eq!(sum_1_to(100), 5050);
}
