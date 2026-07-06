use sol_08_06_add_struct_lifetime::Tag;

#[test]
fn tags() {
    let t = Tag::new("rust");
    assert_eq!(t.name(), "rust");
    assert_eq!(t.name, "rust");
}
