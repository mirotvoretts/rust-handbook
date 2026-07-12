use ex_29_03_c_strings::*;

#[test]
fn strlen_matches() {
    let c = to_c("hello").unwrap();
    assert_eq!(c_strlen(&c), 5);
}

#[test]
fn empty_string() {
    let c = to_c("").unwrap();
    assert_eq!(c_strlen(&c), 0);
}

#[test]
fn interior_nul_rejected() {
    assert!(to_c("a\0b").is_err());
}

#[test]
fn read_back() {
    let c = to_c("world").unwrap();
    let s = unsafe { from_c_ptr(c.as_ptr()) };
    assert_eq!(s, "world");
}
