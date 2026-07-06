use sol_10_03_paths_super::{base_value, child};

#[test]
fn base() {
    assert_eq!(base_value(), 10);
}

#[test]
fn child_uses_super() {
    assert_eq!(child::plus_base(5), 15);
    assert_eq!(child::plus_base(0), 10);
    assert_eq!(child::plus_base(-3), 7);
}
