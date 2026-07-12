use sol_30_06_record::record;

record!(Point { x: i32, y: i32 });
record!(Named {
    id: u32,
    label: String,
});

#[test]
fn point_new_and_fields() {
    let p = Point::new(1, 2);
    assert_eq!(p.x, 1);
    assert_eq!(p.y, 2);
}

#[test]
fn derives_work() {
    let p = Point::new(3, 4);
    assert_eq!(p.clone(), p);
    assert_eq!(p, Point { x: 3, y: 4 });
    assert_eq!(format!("{:?}", p), "Point { x: 3, y: 4 }");
}

#[test]
fn named_record() {
    let n = Named::new(7, String::from("hi"));
    assert_eq!(n.id, 7);
    assert_eq!(n.label, "hi");
}
