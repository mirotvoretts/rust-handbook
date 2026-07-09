use ex_21_03_rc_refcell_counter::{handle, inc, new_counter, value};
use std::rc::Rc;

#[test]
fn handles_share_one_value() {
    let a = new_counter();
    let b = handle(&a);
    let c = handle(&a);
    inc(&a);
    inc(&b);
    inc(&c);
    // мутация через любую ручку видна через все
    assert_eq!(value(&a), 3);
    assert_eq!(value(&b), 3);
    assert_eq!(value(&c), 3);
}

#[test]
fn handle_bumps_strong_count() {
    let a = new_counter();
    assert_eq!(Rc::strong_count(&a), 1);
    let b = handle(&a);
    assert_eq!(Rc::strong_count(&a), 2);
    drop(b);
    assert_eq!(Rc::strong_count(&a), 1);
}

#[test]
fn independent_counters_do_not_share() {
    let a = new_counter();
    let b = new_counter();
    inc(&a);
    assert_eq!(value(&a), 1);
    assert_eq!(value(&b), 0);
}
