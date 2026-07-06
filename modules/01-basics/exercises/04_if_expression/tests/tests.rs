use ex_01_04_if_expression::{abs_diff, grade, sign};

#[test]
fn signs() {
    assert_eq!(sign(-5), -1);
    assert_eq!(sign(0), 0);
    assert_eq!(sign(9), 1);
}

#[test]
fn grades() {
    assert_eq!(grade(95), 'A');
    assert_eq!(grade(90), 'A');
    assert_eq!(grade(85), 'B');
    assert_eq!(grade(72), 'C');
    assert_eq!(grade(60), 'D');
    assert_eq!(grade(40), 'F');
}

#[test]
fn abs_differences() {
    assert_eq!(abs_diff(2, 7), 5);
    assert_eq!(abs_diff(7, 2), 5);
    assert_eq!(abs_diff(-3, 3), 6);
}
