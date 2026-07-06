use sol_03_04_shadowing::{shadow_math, trimmed_len};

#[test]
fn math() {
    assert_eq!(shadow_math(0), 9);
    assert_eq!(shadow_math(3), 15);
    assert_eq!(shadow_math(-5), -1);
}

#[test]
fn trimmed() {
    assert_eq!(trimmed_len("  hi  "), 2);
    assert_eq!(trimmed_len("abc"), 3);
    assert_eq!(trimmed_len("   "), 0);
    assert_eq!(trimmed_len("  привет "), 6);
}
