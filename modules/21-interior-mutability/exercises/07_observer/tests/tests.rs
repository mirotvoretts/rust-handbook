use ex_21_07_observer::{Observer, Subject};
use std::cell::RefCell;
use std::rc::Rc;

fn observer(start: i32) -> Observer {
    Rc::new(RefCell::new(start))
}

#[test]
fn notifies_all_living() {
    let mut s = Subject::new();
    let a = observer(0);
    let b = observer(10);
    s.subscribe(&a);
    s.subscribe(&b);
    assert_eq!(s.notify(5), 2);
    assert_eq!(*a.borrow(), 5);
    assert_eq!(*b.borrow(), 15);
}

#[test]
fn dead_observer_is_skipped() {
    let mut s = Subject::new();
    let a = observer(0);
    let b = observer(10);
    s.subscribe(&a);
    s.subscribe(&b);
    drop(b); // b мёртв: subject держал только Weak
    assert_eq!(s.notify(1), 1); // уведомлён лишь a
    assert_eq!(*a.borrow(), 1);
}

#[test]
fn subscribe_does_not_keep_observer_alive() {
    let mut s = Subject::new();
    let a = observer(0);
    s.subscribe(&a);
    // единственная сильная ссылка - переменная a
    assert_eq!(Rc::strong_count(&a), 1);
    drop(a);
    // все наблюдатели мертвы -> уведомлять некого
    assert_eq!(s.notify(100), 0);
}
