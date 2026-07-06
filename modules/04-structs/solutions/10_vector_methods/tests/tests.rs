use sol_04_10_vector_methods::Vector2;

#[test]
fn addition() {
    let a = Vector2::new(1.0, 2.0);
    let b = Vector2::new(3.0, -1.0);
    assert_eq!(a.add(&b), Vector2::new(4.0, 1.0));
    assert_eq!(a, Vector2::new(1.0, 2.0));
}

#[test]
fn scaling() {
    assert_eq!(Vector2::new(2.0, -3.0).scale(2.0), Vector2::new(4.0, -6.0));
}

#[test]
fn dot_product() {
    assert!((Vector2::new(1.0, 2.0).dot(&Vector2::new(3.0, 4.0)) - 11.0).abs() < 1e-9);
}

#[test]
fn length() {
    assert!((Vector2::new(3.0, 4.0).length() - 5.0).abs() < 1e-9);
    assert!((Vector2::new(0.0, 0.0).length()).abs() < 1e-9);
}
