use sol_08_01_elided::{head, without_head};

#[test]
fn heads() {
    assert_eq!(head("hello"), "h");
    assert_eq!(head(""), "");
    assert_eq!(head("привет"), "п");
}

#[test]
fn rests() {
    assert_eq!(without_head("hello"), "ello");
    assert_eq!(without_head("я"), "");
    assert_eq!(without_head(""), "");
    assert_eq!(without_head("привет"), "ривет");
}
