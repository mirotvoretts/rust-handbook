use sol_01_02_const_values::{circle_area, minutes_to_seconds, seconds_in_days};

#[test]
fn days_to_seconds() {
    assert_eq!(seconds_in_days(1), 86_400);
    assert_eq!(seconds_in_days(2), 172_800);
    assert_eq!(seconds_in_days(0), 0);
}

#[test]
fn minutes() {
    assert_eq!(minutes_to_seconds(3), 180);
    assert_eq!(minutes_to_seconds(0), 0);
}

#[test]
fn area() {
    assert!((circle_area(2.0) - 12.566_370_614_359_172).abs() < 1e-9);
    assert!((circle_area(0.0)).abs() < 1e-9);
}
