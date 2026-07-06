use ex_02_02_overflow_ops::{checked_add_u8, saturating_sub_u8, wrapping_add_u8};

#[test]
fn wrapping() {
    assert_eq!(wrapping_add_u8(1, 2), 3);
    assert_eq!(wrapping_add_u8(250, 10), 4);
    assert_eq!(wrapping_add_u8(255, 1), 0);
}

#[test]
fn checked() {
    assert_eq!(checked_add_u8(1, 2), Some(3));
    assert_eq!(checked_add_u8(250, 10), None);
    assert_eq!(checked_add_u8(255, 0), Some(255));
}

#[test]
fn saturating() {
    assert_eq!(saturating_sub_u8(10, 3), 7);
    assert_eq!(saturating_sub_u8(3, 10), 0);
    assert_eq!(saturating_sub_u8(0, 0), 0);
}
