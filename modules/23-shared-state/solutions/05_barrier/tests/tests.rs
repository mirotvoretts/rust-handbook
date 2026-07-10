use sol_23_05_barrier::Barrier;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

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
    b.wait();
}
