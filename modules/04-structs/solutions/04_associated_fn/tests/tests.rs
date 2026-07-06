use sol_04_04_associated_fn::Rectangle;

#[test]
fn new_rect() {
    let r = Rectangle::new(3, 4);
    assert_eq!(r.width, 3);
    assert_eq!(r.height, 4);
}

#[test]
fn square() {
    let s = Rectangle::square(5);
    assert_eq!(s.width, 5);
    assert_eq!(s.height, 5);
}
