use ex_06_01_move_basics::{concat, shout, total_len};

#[test]
fn concat_joins() {
    assert_eq!(concat(String::from("foo"), String::from("bar")), "foobar");
    assert_eq!(concat(String::new(), String::from("x")), "x");
}

#[test]
fn total_len_sums() {
    let items = vec![String::from("ab"), String::from("cde"), String::new()];
    assert_eq!(total_len(items), 5);
}

#[test]
fn shout_uppercases() {
    assert_eq!(shout(String::from("Rust")), "RUST");
}
