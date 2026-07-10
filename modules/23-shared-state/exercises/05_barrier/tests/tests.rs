use ex_23_05_barrier::Barrier;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// Инвариант барьера: ни один поток не проходит wait(), пока все N не прибыли.
/// Каждый поток до барьера увеличивает `arrived`, после барьера читает его -
/// и обязан увидеть ровно N (все успели прибыть до того, как хоть кого-то пустили).
#[test]
fn all_arrive_before_any_passes() {
    let n = 8;
    let barrier = Arc::new(Barrier::new(n));
    let arrived = Arc::new(AtomicUsize::new(0));

    let mut hs = Vec::new();
    for _ in 0..n {
        let b = Arc::clone(&barrier);
        let a = Arc::clone(&arrived);
        hs.push(thread::spawn(move || {
            a.fetch_add(1, Ordering::SeqCst);
            b.wait();
            a.load(Ordering::SeqCst)
        }));
    }

    for h in hs {
        assert_eq!(h.join().unwrap(), n);
    }
}

#[test]
fn single_participant() {
    let b = Barrier::new(1);
    b.wait(); // не должно заблокироваться
}

#[test]
fn two_participants() {
    let barrier = Arc::new(Barrier::new(2));
    let passed = Arc::new(AtomicUsize::new(0));

    let mut hs = Vec::new();
    for _ in 0..2 {
        let b = Arc::clone(&barrier);
        let p = Arc::clone(&passed);
        hs.push(thread::spawn(move || {
            b.wait();
            p.fetch_add(1, Ordering::SeqCst);
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(passed.load(Ordering::SeqCst), 2);
}
