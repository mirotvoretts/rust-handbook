use sol_06_04_drop_prints::Noisy;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn locals_drop_in_reverse_order() {
    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let _a = Noisy::new(1, Rc::clone(&log));
        let _b = Noisy::new(2, Rc::clone(&log));
        let _c = Noisy::new(3, Rc::clone(&log));
    } // роняются c, b, a
    assert_eq!(*log.borrow(), vec![3, 2, 1]);
}

#[test]
fn vec_elements_drop_in_order() {
    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let _v = vec![
            Noisy::new(1, Rc::clone(&log)),
            Noisy::new(2, Rc::clone(&log)),
            Noisy::new(3, Rc::clone(&log)),
        ];
    } // элементы Vec роняются в прямом порядке
    assert_eq!(*log.borrow(), vec![1, 2, 3]);
}

#[test]
fn explicit_drop_happens_immediately() {
    let log = Rc::new(RefCell::new(Vec::new()));
    let a = Noisy::new(1, Rc::clone(&log));
    let _b = Noisy::new(2, Rc::clone(&log));
    drop(a); // 1 роняется прямо сейчас, ещё до _b
    assert_eq!(*log.borrow(), vec![1]);
}
