use ex_13_04_partial_eq::Username;

fn u(s: &str) -> Username {
    Username(s.to_string())
}

#[test]
fn case_insensitive_between_usernames() {
    assert_eq!(u("Alice"), u("alice"));
    assert_eq!(u("BOB"), u("bob"));
    assert!(u("alice") != u("bob"));
}

#[test]
fn ne_comes_from_default_method() {
    assert!(u("x") != u("y"));
}

#[test]
fn compare_with_str() {
    assert!(u("Alice") == *"aLiCe");
    assert!(u("Alice") != *"bob");
}
