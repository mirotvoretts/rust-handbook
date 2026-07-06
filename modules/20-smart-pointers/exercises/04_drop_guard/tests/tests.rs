use ex_20_04_drop_guard::{dropped, Guard};
use std::rc::Rc;
use std::sync::atomic::AtomicUsize;

#[test]
fn drops_at_scope_end() {
    let counter = Rc::new(AtomicUsize::new(0));
    {
        let _a = Guard::new(Rc::clone(&counter));
        let _b = Guard::new(Rc::clone(&counter));
        assert_eq!(dropped(&counter), 0); // оба ещё живы
    }
    assert_eq!(dropped(&counter), 2); // область закрылась - оба дропнуты
}

#[test]
fn explicit_early_drop() {
    let counter = Rc::new(AtomicUsize::new(0));
    let g = Guard::new(Rc::clone(&counter));
    assert_eq!(dropped(&counter), 0);
    drop(g); // std::mem::drop уничтожает сейчас
    assert_eq!(dropped(&counter), 1);
}

#[test]
fn independent_counters() {
    let c1 = Rc::new(AtomicUsize::new(0));
    let c2 = Rc::new(AtomicUsize::new(0));
    drop(Guard::new(Rc::clone(&c1)));
    assert_eq!(dropped(&c1), 1);
    assert_eq!(dropped(&c2), 0);
}
