use ex_05_05_shape_area::{area, Shape};

#[test]
fn circle() {
    let a = area(&Shape::Circle(2.0));
    assert!((a - std::f64::consts::PI * 4.0).abs() < 1e-9);
}

#[test]
fn rectangle() {
    assert!((area(&Shape::Rectangle(3.0, 4.0)) - 12.0).abs() < 1e-9);
}

#[test]
fn triangle() {
    let t = Shape::Triangle {
        base: 6.0,
        height: 4.0,
    };
    assert!((area(&t) - 12.0).abs() < 1e-9);
}
