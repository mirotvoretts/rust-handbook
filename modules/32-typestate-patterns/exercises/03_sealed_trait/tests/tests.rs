use ex_32_03_sealed_trait::{describe, Circle, Shape, Square};

#[test]
fn areas() {
    let c = Circle { radius: 2.0 };
    let s = Square { side: 3.0 };
    assert!((c.area() - 12.566).abs() < 1e-2);
    assert_eq!(s.area(), 9.0);
    assert_eq!(s.name(), "square");
    assert_eq!(describe(&s), "square area=9.00");
}
