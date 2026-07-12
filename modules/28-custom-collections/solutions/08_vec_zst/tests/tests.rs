use sol_28_08_vec_zst::*;

use std::sync::atomic::{AtomicUsize, Ordering};

static DROPS: AtomicUsize = AtomicUsize::new(0);

// ZST с деструктором: size_of == 0, но Drop считает вызовы.
struct Tick;
impl Drop for Tick {
    fn drop(&mut self) {
        DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn zst_no_memory_but_correct_drops() {
    assert_eq!(std::mem::size_of::<Tick>(), 0);

    let start = DROPS.load(Ordering::SeqCst);
    let mut v: ZstVec<Tick> = ZstVec::new();
    for _ in 0..1000 {
        v.push(Tick);
    }
    assert_eq!(v.len(), 1000);
    assert!(!v.is_empty());
    // Ёмкость "бесконечна": память не выделяется.
    assert_eq!(v.capacity(), usize::MAX);

    // push не должен ронять элементы досрочно.
    assert_eq!(DROPS.load(Ordering::SeqCst) - start, 0);

    // Три pop - три деструктора (возвращённые Tick умирают в конце строки).
    v.pop();
    v.pop();
    v.pop();
    assert_eq!(DROPS.load(Ordering::SeqCst) - start, 3);
    assert_eq!(v.len(), 997);

    drop(v); // остальные 997
    assert_eq!(DROPS.load(Ordering::SeqCst) - start, 1000);
}

#[test]
fn empty_pop() {
    let mut v: ZstVec<()> = ZstVec::new();
    assert!(v.is_empty());
    assert!(v.pop().is_none());
}

#[test]
#[should_panic]
fn rejects_non_zst() {
    // u32 - не ZST: конструктор обязан отвергнуть такой тип.
    let _v: ZstVec<u32> = ZstVec::new();
}
