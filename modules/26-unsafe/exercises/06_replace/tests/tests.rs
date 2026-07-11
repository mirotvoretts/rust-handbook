use ex_26_06_replace::replace;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[test]
fn returns_old_sets_new() {
    let mut v = 10;
    let old = replace(&mut v, 99);
    assert_eq!(old, 10);
    assert_eq!(v, 99);
}

#[test]
fn works_with_strings() {
    let mut s = String::from("old");
    let old = replace(&mut s, String::from("new"));
    assert_eq!(old, "old");
    assert_eq!(s, "new");
}

// Тип, считающий свои дропы: ловит double-free и утечки.
struct Counted(Arc<AtomicUsize>);
impl Drop for Counted {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn each_value_dropped_exactly_once() {
    let drops = Arc::new(AtomicUsize::new(0));
    let mut a = Counted(drops.clone());
    let b = Counted(drops.clone());

    let old = replace(&mut a, b); // old - прежнее a
    assert_eq!(drops.load(Ordering::SeqCst), 0, "пока ничего не дропнуто");

    drop(old); // дропаем прежнее a -> 1
    assert_eq!(drops.load(Ordering::SeqCst), 1);

    drop(a); // дропаем новое значение (бывшее b) -> 2
    assert_eq!(drops.load(Ordering::SeqCst), 2);
}
