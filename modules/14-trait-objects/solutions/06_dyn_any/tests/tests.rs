use std::any::Any;
use sol_14_06_dyn_any::sort_bag;

#[test]
fn sorts_mixed_bag() {
    let bag: Vec<Box<dyn Any>> = vec![
        Box::new(1i32),
        Box::new(String::from("ab")),
        Box::new(2i32),
        Box::new(3.5f64),
        Box::new(String::from("cd")),
    ];
    let s = sort_bag(bag);
    assert_eq!(s.int_sum, 3);
    assert_eq!(s.strings_joined, "abcd");
    assert_eq!(s.unknown_count, 1);
}

#[test]
fn i64_is_not_i32() {
    let bag: Vec<Box<dyn Any>> = vec![Box::new(100i64)]; // downcast различает типы точно
    let s = sort_bag(bag);
    assert_eq!(s.int_sum, 0);
    assert_eq!(s.unknown_count, 1);
}

#[test]
fn empty_bag() {
    let s = sort_bag(vec![]);
    assert_eq!(s.int_sum, 0);
    assert_eq!(s.strings_joined, "");
    assert_eq!(s.unknown_count, 0);
}
