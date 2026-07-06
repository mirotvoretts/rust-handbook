use ex_05_06_if_let::{is_some_even, label};

#[test]
fn labels() {
    assert_eq!(label(Some(42)), "value: 42");
    assert_eq!(label(None), "none");
}

#[test]
fn even_inside() {
    assert!(is_some_even(Some(4)));
    assert!(!is_some_even(Some(5)));
    assert!(!is_some_even(None));
}
