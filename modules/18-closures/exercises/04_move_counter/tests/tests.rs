use ex_18_04_move_counter::{make_adder, make_counter};

#[test]
fn counter_increments_from_one() {
    let mut c = make_counter();
    assert_eq!(c(), 1);
    assert_eq!(c(), 2);
    assert_eq!(c(), 3);
}

#[test]
fn counters_are_independent() {
    let mut a = make_counter();
    let mut b = make_counter();
    assert_eq!(a(), 1);
    assert_eq!(a(), 2);
    assert_eq!(b(), 1); // b со своим счётчиком, не задет вызовами a
}

#[test]
fn adder_accumulates_from_start() {
    let mut acc = make_adder(10);
    assert_eq!(acc(5), 15);
    assert_eq!(acc(-3), 12);
    assert_eq!(acc(0), 12);
}
