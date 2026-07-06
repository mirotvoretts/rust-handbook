use sol_06_10_no_double_free::{relay, Resource};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn freed_exactly_once_after_many_moves() {
    let freed = Rc::new(RefCell::new(Vec::new()));
    {
        let r = Resource::new(42, Rc::clone(&freed));
        let r = relay(r); // move
        let r = relay(r); // move ещё раз
        let _r = relay(r); // и ещё
                           // журнал пока пуст: ни одного Drop не случилось
        assert!(freed.borrow().is_empty());
    } // здесь единственный живой владелец роняется
    assert_eq!(*freed.borrow(), vec![42]); // ровно один free, не несколько
}

#[test]
fn each_resource_freed_once() {
    let freed = Rc::new(RefCell::new(Vec::new()));
    {
        let _a = Resource::new(1, Rc::clone(&freed));
        let _b = relay(Resource::new(2, Rc::clone(&freed)));
    }
    let mut got = freed.borrow().clone();
    got.sort();
    assert_eq!(got, vec![1, 2]);
}
