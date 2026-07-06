use sol_05_07_option_combinators::{double_opt, keep_positive, or_zero};

#[test]
fn mapping() {
    assert_eq!(double_opt(Some(21)), Some(42));
    assert_eq!(double_opt(None), None);
}

#[test]
fn defaulting() {
    assert_eq!(or_zero(Some(7)), 7);
    assert_eq!(or_zero(None), 0);
}

#[test]
fn filtering() {
    assert_eq!(keep_positive(Some(5)), Some(5));
    assert_eq!(keep_positive(Some(-5)), None);
    assert_eq!(keep_positive(Some(0)), None);
    assert_eq!(keep_positive(None), None);
}
