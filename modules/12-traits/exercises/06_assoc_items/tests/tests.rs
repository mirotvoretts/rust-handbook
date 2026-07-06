use ex_12_06_assoc_items::{snap_to_bound, Bounded};

#[test]
fn constants() {
    assert_eq!(i8::MIN_VALUE, -128);
    assert_eq!(i8::MAX_VALUE, 127);
    assert_eq!(u8::MIN_VALUE, 0);
    assert_eq!(u8::MAX_VALUE, 255);
}

#[test]
fn midpoint_dispatches_on_return_type() {
    let m: i8 = Bounded::midpoint();
    assert_eq!(m, -1);
    assert_eq!(<u8 as Bounded>::midpoint(), 127); // у u8 есть inherent midpoint(a, b) — нужен fully qualified
}

#[test]
fn snapping() {
    assert_eq!(snap_to_bound(-100i8), -128);
    assert_eq!(snap_to_bound(100i8), 127);
    assert_eq!(snap_to_bound(5u8), 0);
    assert_eq!(snap_to_bound(200u8), 255);
}
