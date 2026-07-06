use ex_13_02_display::Celsius;

#[test]
fn formats_with_one_decimal() {
    assert_eq!(format!("{}", Celsius(36.6)), "36.6°C");
    assert_eq!(format!("{}", Celsius(0.0)), "0.0°C");
}

#[test]
fn negative_and_rounding() {
    assert_eq!(format!("{}", Celsius(-5.0)), "-5.0°C");
    assert_eq!(format!("{}", Celsius(36.649)), "36.6°C");
}

#[test]
fn to_string_comes_for_free() {
    // blanket impl ToString для всех T: Display
    assert_eq!(Celsius(100.0).to_string(), "100.0°C");
}
