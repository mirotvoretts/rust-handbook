use ex_14_01_first_dyn::{total_area, Circle, Rect, Shape};

#[test]
fn areas() {
    assert!((Circle { r: 1.0 }.area() - std::f64::consts::PI).abs() < 1e-9);
    assert_eq!(Rect { w: 2.0, h: 3.0 }.area(), 6.0);
}

#[test]
fn heterogeneous_vec() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rect { w: 1.0, h: 1.0 }),
        Box::new(Rect { w: 2.0, h: 2.0 }),
        Box::new(Circle { r: 0.0 }),
    ];
    assert_eq!(total_area(&shapes), 5.0);
}

#[test]
fn fat_pointer_is_two_words() {
    use std::mem::size_of;
    assert_eq!(size_of::<Box<dyn Shape>>(), 2 * size_of::<usize>());
    assert_eq!(size_of::<Box<Circle>>(), size_of::<usize>());
}
