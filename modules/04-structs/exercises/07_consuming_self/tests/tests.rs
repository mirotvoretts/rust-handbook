use ex_04_07_consuming_self::{Temperature, Wrapper};

#[test]
fn conversions() {
    assert!((Temperature::new(100.0).into_fahrenheit() - 212.0).abs() < 1e-9);
    assert!((Temperature::new(0.0).into_fahrenheit() - 32.0).abs() < 1e-9);
    assert!((Temperature::new(0.0).into_kelvin() - 273.15).abs() < 1e-9);
}

#[test]
fn extract_inner() {
    let w = Wrapper::new(String::from("привет"));
    let s = w.into_inner();
    assert_eq!(s, "привет");
    // w потреблён - обращаться к нему уже нельзя (проверено компилятором)
}
