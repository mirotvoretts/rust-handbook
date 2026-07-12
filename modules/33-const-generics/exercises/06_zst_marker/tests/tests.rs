use ex_33_06_zst_marker::{is_zst, Marker, TinySet, UnitCounter};

#[test]
fn zsts_have_zero_size() {
    assert!(is_zst::<Marker>());
    assert!(is_zst::<()>());
    assert!(!is_zst::<u8>());
    assert_eq!(std::mem::size_of::<Marker>(), 0);
}

#[test]
fn tiny_set() {
    let mut s = TinySet::new();
    assert!(s.insert("a"));
    assert!(s.insert("b"));
    assert!(!s.insert("a"));
    assert_eq!(s.len(), 2);
    assert!(s.contains(&"a"));
    assert!(!s.contains(&"z"));
    assert!(!s.is_empty());
}

#[test]
fn unit_counter() {
    let mut c = UnitCounter::new();
    c.tick();
    c.tick();
    c.tick();
    assert_eq!(c.count(), 3);
}
