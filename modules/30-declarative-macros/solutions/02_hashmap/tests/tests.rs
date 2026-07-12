use sol_30_02_hashmap::hashmap;
use std::collections::HashMap;

#[test]
fn empty() {
    let m: HashMap<&str, i32> = hashmap! {};
    assert!(m.is_empty());
}

#[test]
fn builds_pairs() {
    let m = hashmap! { "a" => 1, "b" => 2 };
    assert_eq!(m.len(), 2);
    assert_eq!(m.get("a"), Some(&1));
    assert_eq!(m.get("b"), Some(&2));
}

#[test]
fn trailing_comma() {
    let m = hashmap! { "x" => 10, };
    assert_eq!(m["x"], 10);
}
