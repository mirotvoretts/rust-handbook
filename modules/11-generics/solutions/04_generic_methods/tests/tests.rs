use sol_11_04_generic_methods::Holder;

#[test]
fn new_get_into() {
    let h = Holder::new(7);
    assert_eq!(*h.get(), 7);
    assert_eq!(h.into_inner(), 7);
}

#[test]
fn holds_owned_values() {
    let h = Holder::new(String::from("data"));
    assert_eq!(h.get().len(), 4);
    assert_eq!(h.into_inner(), "data");
}

fn double(n: i32) -> i32 { n * 2 }

#[test]
fn map_with_fn_pointer() {
    let h = Holder::new(21).map(double);
    assert_eq!(h.into_inner(), 42);
}

#[test]
fn map_changes_type() {
    let h: Holder<usize> = Holder::new("hello").map(str::len);
    assert_eq!(h.into_inner(), 5);
}
