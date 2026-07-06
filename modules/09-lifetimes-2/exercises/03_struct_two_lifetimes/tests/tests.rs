use ex_09_03_struct_two_lifetimes::Report;

#[test]
fn fields_and_methods() {
    let data = [1, 2, 3];
    let r = Report::new("Sales", &data);
    assert_eq!(r.title(), "Sales");
    assert_eq!(r.sum(), 6);
    assert_eq!(r.header(), "Sales (3)");
}

#[test]
fn empty_data() {
    let r = Report::new("Empty", &[]);
    assert_eq!(r.sum(), 0);
    assert_eq!(r.header(), "Empty (0)");
}
