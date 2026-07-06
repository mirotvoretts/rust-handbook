use ex_20_06_mybox_deref::MyBox;

#[test]
fn deref_reads_value() {
    let b = MyBox::new(5);
    assert_eq!(*b, 5);
}

#[test]
fn deref_mut_writes_value() {
    let mut m = MyBox::new(10);
    *m += 5;
    assert_eq!(*m, 15);
}

fn takes_str(s: &str) -> usize {
    s.len()
}

#[test]
fn deref_coercion_to_str() {
    let s = MyBox::new(String::from("hello"));
    // &MyBox<String> -> &String -> &str автоматически
    assert_eq!(takes_str(&s), 5);
}

#[test]
fn methods_of_inner_through_deref() {
    let s = MyBox::new(String::from("abc"));
    assert_eq!(s.len(), 3); // метод str через Deref
    assert_eq!(s.chars().next(), Some('a'));
}
