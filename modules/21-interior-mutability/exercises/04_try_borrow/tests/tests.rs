use ex_21_04_try_borrow::{add_checked, read_if_free};
use std::cell::RefCell;

#[test]
fn reads_when_free() {
    let c = RefCell::new(3);
    assert_eq!(read_if_free(&c), Some(3));
}

#[test]
fn read_fails_while_mutably_borrowed() {
    let c = RefCell::new(3);
    let guard = c.borrow_mut(); // держим активный &mut
    assert_eq!(read_if_free(&c), None); // занято - но НЕ паника
    drop(guard);
    assert_eq!(read_if_free(&c), Some(3)); // заём отпущен
}

#[test]
fn add_checked_ok_when_free() {
    let c = RefCell::new(10);
    assert_eq!(add_checked(&c, 5), Ok(15));
    assert_eq!(*c.borrow(), 15);
}

#[test]
fn add_checked_err_while_borrowed() {
    let c = RefCell::new(10);
    let guard = c.borrow(); // активный разделяемый заём
    assert_eq!(add_checked(&c, 5), Err(()));
    drop(guard);
    assert_eq!(add_checked(&c, 5), Ok(15));
}
