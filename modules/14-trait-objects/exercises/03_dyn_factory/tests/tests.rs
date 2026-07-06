use ex_14_03_dyn_factory::make_shape;

#[test]
fn makes_known_shapes() {
    let c = make_shape("circle").unwrap();
    assert_eq!(c.name(), "circle");
    assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);

    let s = make_shape("square").unwrap();
    assert_eq!(s.name(), "square");
    assert_eq!(s.area(), 1.0);
}

#[test]
fn unknown_kind_is_none() {
    assert!(make_shape("triangle").is_none());
    assert!(make_shape("").is_none());
}

#[test]
fn runtime_driven_collection() {
    let kinds = ["circle", "nope", "square", "square"];
    let mut shapes = Vec::new();
    for k in kinds {
        if let Some(s) = make_shape(k) {
            shapes.push(s);
        }
    }
    assert_eq!(shapes.len(), 3);
}
