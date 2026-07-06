use ex_04_03_methods::Rectangle;

#[test]
fn area() {
    let r = Rectangle { width: 3, height: 4 };
    assert_eq!(r.area(), 12);
}

#[test]
fn perimeter() {
    let r = Rectangle { width: 3, height: 4 };
    assert_eq!(r.perimeter(), 14);
}

#[test]
fn square_check() {
    let sq = Rectangle { width: 5, height: 5 };
    let rect = Rectangle { width: 5, height: 6 };
    assert!(sq.is_square());
    assert!(!rect.is_square());
}
