use sol_04_11_nested_struct::{Line, Point};

fn p(x: f64, y: f64) -> Point {
    Point { x, y }
}

#[test]
fn length() {
    let l = Line::new(p(0.0, 0.0), p(3.0, 4.0));
    assert!((l.length() - 5.0).abs() < 1e-9);
}

#[test]
fn midpoint() {
    let l = Line::new(p(0.0, 0.0), p(4.0, 2.0));
    assert_eq!(l.midpoint(), p(2.0, 1.0));

    let l2 = Line::new(p(-1.0, -1.0), p(1.0, 1.0));
    assert_eq!(l2.midpoint(), p(0.0, 0.0));
}
