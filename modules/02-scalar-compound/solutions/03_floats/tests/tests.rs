use sol_02_03_floats::{average, hypotenuse, is_close};

#[test]
fn averages() {
    assert!((average(2.0, 4.0) - 3.0).abs() < 1e-9);
    assert!((average(1.0, 2.0) - 1.5).abs() < 1e-9);
}

#[test]
fn closeness() {
    assert!(is_close(1.0, 1.000_000_1, 1e-3));
    assert!(!is_close(1.0, 2.0, 0.1));
    assert!(is_close(0.1 + 0.2, 0.3, 1e-9));
    assert!(0.1 + 0.2 != 0.3);
}

#[test]
fn hypot() {
    assert!((hypotenuse(3.0, 4.0) - 5.0).abs() < 1e-9);
    assert!((hypotenuse(0.0, 0.0)).abs() < 1e-9);
}
