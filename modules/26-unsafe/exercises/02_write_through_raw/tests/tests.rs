use ex_26_02_write_through_raw::set_via_raw;

#[test]
fn overwrites() {
    let mut v = 10;
    set_via_raw(&mut v, 99);
    assert_eq!(v, 99);
}

#[test]
fn to_negative() {
    let mut v = 0;
    set_via_raw(&mut v, -5);
    assert_eq!(v, -5);
}

#[test]
fn repeated() {
    let mut v = 1;
    set_via_raw(&mut v, 2);
    set_via_raw(&mut v, 3);
    assert_eq!(v, 3);
}
