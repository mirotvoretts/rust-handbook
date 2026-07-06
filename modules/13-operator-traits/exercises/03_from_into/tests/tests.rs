use ex_13_03_from_into::{Centimeters, Feet, Meters};

#[test]
fn from_direct() {
    assert_eq!(Centimeters::from(Meters(1.5)), Centimeters(150.0));
    assert_eq!(Meters::from(Centimeters(250.0)), Meters(2.5));
}

#[test]
fn into_is_free() {
    let cm: Centimeters = Meters(2.0).into();
    assert_eq!(cm, Centimeters(200.0));
    let m: Meters = Feet(10.0).into();
    assert!((m.0 - 3.048).abs() < 1e-9);
}

fn takes_meters(m: impl Into<Meters>) -> f64 {
    m.into().0
}

#[test]
fn generic_api_accepts_all_units() {
    assert!((takes_meters(Feet(1.0)) - 0.3048).abs() < 1e-9);
    assert_eq!(takes_meters(Centimeters(100.0)), 1.0);
    assert_eq!(takes_meters(Meters(5.0)), 5.0); // From<T> for T есть в std
}
