use sol_13_07_scalar_mul::{lerp, Vec2};

fn v(x: f64, y: f64) -> Vec2 {
    Vec2 { x, y }
}

#[test]
fn scalar_multiplication() {
    assert_eq!(v(1.0, -2.0) * 3.0, v(3.0, -6.0));
    assert_eq!(v(1.0, 1.0) * 0.0, v(0.0, 0.0));
}

#[test]
fn negation() {
    assert_eq!(-v(1.0, -2.0), v(-1.0, 2.0));
}

#[test]
fn lerp_endpoints_and_middle() {
    let a = v(0.0, 0.0);
    let b = v(10.0, 20.0);
    assert_eq!(lerp(a, b, 0.0), a);
    assert_eq!(lerp(a, b, 1.0), b);
    assert_eq!(lerp(a, b, 0.5), v(5.0, 10.0));
}
