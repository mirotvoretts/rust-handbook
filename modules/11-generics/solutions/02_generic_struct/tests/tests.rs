use sol_11_02_generic_struct::{into_tuple, make_pair, Pair};

#[test]
fn ints() {
    let p = make_pair(1, 2);
    assert_eq!(p.left, 1);
    assert_eq!(p.right, 2);
}

#[test]
fn owned_strings_move_through() {
    let p = make_pair(String::from("a"), String::from("b"));
    let (l, r) = into_tuple(p);
    assert_eq!(l, "a");
    assert_eq!(r, "b");
}

#[test]
fn literal_construction() {
    let p = Pair { left: 'x', right: 'y' };
    assert_eq!(into_tuple(p), ('x', 'y'));
}
