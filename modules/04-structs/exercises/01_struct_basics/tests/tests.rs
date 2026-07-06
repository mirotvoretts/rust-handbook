use ex_04_01_struct_basics::{make_point, manhattan, translate};

#[test]
fn construction() {
    let p = make_point(3, -4);
    assert_eq!(p.x, 3);
    assert_eq!(p.y, -4);
}

#[test]
fn distance() {
    assert_eq!(manhattan(&make_point(3, 4)), 7);
    assert_eq!(manhattan(&make_point(-3, 4)), 7);
    assert_eq!(manhattan(&make_point(0, 0)), 0);
}

#[test]
fn translation() {
    let p = translate(&make_point(1, 1), 4, -2);
    assert_eq!(p.x, 5);
    assert_eq!(p.y, -1);
}
