use sol_04_06_tuple_struct::Rgb;

#[test]
fn field_access() {
    let c = Rgb(10, 20, 30);
    assert_eq!(c.0, 10);
    assert_eq!(c.1, 20);
    assert_eq!(c.2, 30);
}

#[test]
fn brightness() {
    assert_eq!(Rgb(10, 20, 30).brightness(), 60);
    assert_eq!(Rgb(255, 255, 255).brightness(), 765);
    assert_eq!(Rgb(0, 0, 0).brightness(), 0);
}

#[test]
fn invert() {
    assert_eq!(Rgb(0, 0, 0).invert(), Rgb(255, 255, 255));
    assert_eq!(Rgb(10, 20, 30).invert(), Rgb(245, 235, 225));
}
