use ex_21_01_cell_basics::{bump, replace_get, take};
use std::cell::Cell;

#[test]
fn bump_increments_through_shared_ref() {
    let c = Cell::new(10);
    bump(&c);
    bump(&c);
    assert_eq!(c.get(), 12);
}

#[test]
fn replace_returns_old_value() {
    let c = Cell::new(7);
    let old = replace_get(&c, 100);
    assert_eq!(old, 7);
    assert_eq!(c.get(), 100);
}

#[test]
fn take_leaves_default() {
    let c = Cell::new(42);
    assert_eq!(take(&c), 42);
    assert_eq!(c.get(), 0);
}
