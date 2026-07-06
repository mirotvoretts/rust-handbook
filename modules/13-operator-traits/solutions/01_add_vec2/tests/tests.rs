use sol_13_01_add_vec2::Vec2;

fn v(x: f64, y: f64) -> Vec2 {
    Vec2 { x, y }
}

#[test]
fn addition() {
    assert_eq!(v(1.0, 2.0) + v(3.0, 4.0), v(4.0, 6.0));
}

#[test]
fn subtraction() {
    assert_eq!(v(5.0, 5.0) - v(1.0, 2.0), v(4.0, 3.0));
}

#[test]
fn operators_chain() {
    let a = v(1.0, 1.0);
    assert_eq!(a + a + a - a, v(2.0, 2.0));
}
