use sol_08_05_excerpt_struct::Excerpt;

#[test]
fn holds_reference() {
    let text = String::from("hello world");
    let e = Excerpt::new(&text[..5]);
    assert_eq!(e.part(), "hello");
    assert_eq!(e.len(), 5);
    assert_eq!(e.part, "hello");
}

#[test]
fn whole_string() {
    let e = Excerpt::new("rust");
    assert_eq!(e.part(), "rust");
    assert_eq!(e.len(), 4);
}
