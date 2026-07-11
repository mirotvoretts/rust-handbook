use ex_26_07_stack::Stack;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn push_pop_lifo() {
    let mut s: Stack<i32> = Stack::new();
    assert!(s.is_empty());
    s.push(1).unwrap();
    s.push(2).unwrap();
    s.push(3).unwrap();
    assert_eq!(s.len(), 3);
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
    assert!(s.is_empty());
}

#[test]
fn full_returns_err_without_losing_value() {
    let mut s: Stack<i32> = Stack::new();
    for i in 0..Stack::<i32>::CAP as i32 {
        s.push(i).unwrap();
    }
    assert_eq!(s.len(), Stack::<i32>::CAP);
    assert_eq!(s.push(999), Err(999));
    assert_eq!(s.len(), Stack::<i32>::CAP);
}

struct Counted(Arc<AtomicUsize>);
impl Drop for Counted {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn drop_runs_once_per_remaining_element() {
    let drops = Arc::new(AtomicUsize::new(0));
    {
        let mut s: Stack<Counted> = Stack::new();
        s.push(Counted(drops.clone())).ok().unwrap();
        s.push(Counted(drops.clone())).ok().unwrap();
        s.push(Counted(drops.clone())).ok().unwrap();
        // один снимаем и дропаем явно
        drop(s.pop().unwrap());
        assert_eq!(drops.load(Ordering::SeqCst), 1);
        // остаётся 2 элемента; выход из блока дропает стек
    }
    assert_eq!(drops.load(Ordering::SeqCst), 3);
}

#[test]
fn no_drop_of_uninit_slots() {
    // пустой стек ничего не дропает
    let drops = Arc::new(AtomicUsize::new(0));
    {
        let _s: Stack<Counted> = Stack::new();
    }
    assert_eq!(drops.load(Ordering::SeqCst), 0);
}
