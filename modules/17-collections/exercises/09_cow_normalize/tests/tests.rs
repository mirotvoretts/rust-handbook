use std::borrow::Cow;
use ex_17_09_cow_normalize::{ensure_prefix, snake};

#[test]
fn borrows_when_clean() {
    assert!(matches!(snake("no_spaces"), Cow::Borrowed(_)));
    assert!(matches!(ensure_prefix("id-42"), Cow::Borrowed(_)));
}

#[test]
fn owns_when_modified() {
    let out = snake("a b c");
    assert!(matches!(out, Cow::Owned(_)));
    assert_eq!(out, "a_b_c");

    let out = ensure_prefix("42");
    assert!(matches!(out, Cow::Owned(_)));
    assert_eq!(out, "id-42");
}

#[test]
fn cow_derefs_like_str() {
    let c = snake("x y");
    assert_eq!(c.len(), 3); // методы str через Deref
}
