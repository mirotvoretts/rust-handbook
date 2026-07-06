use ex_10_02_visibility::geometry;

#[test]
fn public_api() {
    assert_eq!(geometry::area(3, 4), 12);
    assert_eq!(geometry::perimeter(3, 4), 14);
}
