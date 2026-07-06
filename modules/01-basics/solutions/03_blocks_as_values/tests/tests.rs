use sol_01_03_blocks_as_values::{average3, celsius_to_fahrenheit, hypot_sq};

#[test]
fn hypotenuse_squared() {
    assert_eq!(hypot_sq(3, 4), 25);
    assert_eq!(hypot_sq(0, 0), 0);
    assert_eq!(hypot_sq(5, 12), 169);
}

#[test]
fn temperature() {
    assert_eq!(celsius_to_fahrenheit(0), 32);
    assert_eq!(celsius_to_fahrenheit(100), 212);
    assert_eq!(celsius_to_fahrenheit(-40), -40);
}

#[test]
fn average() {
    assert_eq!(average3(1, 2, 3), 2);
    assert_eq!(average3(10, 20, 30), 20);
}
