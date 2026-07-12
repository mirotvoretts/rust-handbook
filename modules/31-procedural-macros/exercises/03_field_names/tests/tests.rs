use ex_31_03_field_names::FieldNames;

#[derive(FieldNames)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(FieldNames)]
struct Empty {}

#[test]
fn lists_fields() {
    assert_eq!(Point::field_names(), vec!["x", "y"]);
    assert_eq!(Empty::field_names(), Vec::<&'static str>::new());
}
