use ex_00_06_should_panic::safe_div_100;

#[test]
fn valid_inputs() {
    assert_eq!(safe_div_100(4), 25);
    assert_eq!(safe_div_100(10), 10);
    assert_eq!(safe_div_100(-5), -20);
}

#[test]
#[should_panic]
fn zero_panics() {
    safe_div_100(0);
}
