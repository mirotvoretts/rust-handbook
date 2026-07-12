use ex_29_01_extern_call::*;

#[test]
fn abs_basic() {
    assert_eq!(libc_abs(-5), 5);
    assert_eq!(libc_abs(5), 5);
    assert_eq!(libc_abs(0), 0);
}

#[test]
fn diff() {
    assert_eq!(abs_diff(3, 10), 7);
    assert_eq!(abs_diff(10, 3), 7);
    assert_eq!(abs_diff(-4, 4), 8);
}
