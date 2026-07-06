use ex_06_09_drop_guard::Transaction;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn begin_and_end_bracket_the_scope() {
    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let _tx = Transaction::begin("A", Rc::clone(&log));
        log.borrow_mut().push(String::from("work"));
    } // здесь Drop допишет "end A"
    assert_eq!(*log.borrow(), vec!["begin A", "work", "end A"]);
}

#[test]
fn nested_transactions_end_in_reverse() {
    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let _outer = Transaction::begin("outer", Rc::clone(&log));
        let _inner = Transaction::begin("inner", Rc::clone(&log));
    } // inner роняется раньше outer
    assert_eq!(
        *log.borrow(),
        vec!["begin outer", "begin inner", "end inner", "end outer"]
    );
}

#[test]
fn explicit_drop_ends_early() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let tx = Transaction::begin("X", Rc::clone(&log));
    drop(tx);
    log.borrow_mut().push(String::from("after"));
    assert_eq!(*log.borrow(), vec!["begin X", "end X", "after"]);
}
