use sol_11_07_specialized_impl::Point;

#[test]
fn generic_methods_for_any_type() {
    let p = Point::new("a", "b");
    assert_eq!(p.into_tuple(), ("a", "b"));
    let q = Point::new(1u8, 2u8);
    assert_eq!(q.into_tuple(), (1, 2));
}

#[test]
fn length_only_for_f64() {
    let p = Point::new(3.0, 4.0);
    assert!((p.length() - 5.0).abs() < 1e-9);
}

#[test]
fn midpoint() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(4.0, 6.0);
    let m = a.midpoint(&b);
    assert_eq!((m.x, m.y), (2.0, 3.0));
}
