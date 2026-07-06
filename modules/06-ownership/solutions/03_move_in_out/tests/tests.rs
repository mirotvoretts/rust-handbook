use sol_06_03_move_in_out::{append_bang, make_greeting, split_first};

#[test]
fn greeting() {
    assert_eq!(make_greeting(String::from("Мир")), "Привет, Мир!");
}

#[test]
fn bang() {
    assert_eq!(append_bang(String::from("go")), "go!");
    assert_eq!(append_bang(String::new()), "!");
}

#[test]
fn split() {
    let (first, rest) = split_first(vec![10, 20, 30]);
    assert_eq!(first, Some(10));
    assert_eq!(rest, vec![20, 30]);

    let (none, empty) = split_first(Vec::new());
    assert_eq!(none, None);
    assert_eq!(empty, Vec::<i32>::new());
}
